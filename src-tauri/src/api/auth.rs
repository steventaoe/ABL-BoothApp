use axum::extract::rejection::JsonRejection;
use axum::{
    async_trait,
    body::Body,
    extract::State,
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::{
    state::AppState,
    utils::security::{self, AuthError},
};

// 自定义反序列化函数：接受字符串或数字的i64
fn deserialize_i64_from_str<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{self, Deserialize, Deserializer};

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt {
        String(String),
        Int(i64),
    }

    match StringOrInt::deserialize(deserializer)? {
        StringOrInt::String(s) => {
            if s.is_empty() {
                Ok(None)
            } else {
                s.parse::<i64>()
                    .map(Some)
                    .map_err(|_| de::Error::custom(format!("invalid i64 value: {}", s)))
            }
        }
        StringOrInt::Int(i) => Ok(Some(i)),
    }
}

// 自定义JSON提取器，提供更好的错误诊断
pub struct DebugJson<T>(pub T);

#[async_trait]
impl<T, S> axum::extract::FromRequest<S> for DebugJson<T>
where
    T: serde::de::DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let (parts, body) = req.into_parts();
        let bytes = axum::body::to_bytes(body, usize::MAX)
            .await
            .map_err(|_| (StatusCode::BAD_REQUEST, "Failed to read body".to_string()))?;

        serde_json::from_slice::<T>(&bytes)
            .map(DebugJson)
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid JSON: {}", e)))
    }
}

// 1. 请求体 DTO
#[derive(Deserialize)]
struct LoginRequest {
    role: String, // "admin" | "vendor"
    password: String,
    #[serde(
        rename = "eventId",
        deserialize_with = "deserialize_i64_from_str",
        default
    )] // 接受字符串或数字
    event_id: Option<i64>,
}

// 2. 响应体 DTO
#[derive(Serialize)]
struct LoginResponse {
    message: String,
    role: String,
    access: String,
    #[serde(rename = "eventId", skip_serializing_if = "Option::is_none")]
    event_id: Option<i64>,
    token: String, // 我们把 token 直接放在 Body 里方便前端拿
}

// 3. 路由定义
pub fn router() -> Router<AppState> {
    Router::new().route("/login", post(login_handler))
}

// 4. 处理器逻辑
async fn login_handler(
    State(state): State<AppState>,
    DebugJson(payload): DebugJson<LoginRequest>,
) -> Result<Response, AuthError> {
    // event_id 直接从 payload 中获取
    let event_id = payload.event_id;

    match payload.role.as_str() {
        "admin" => {
            // --- 管理员登录逻辑 ---
            // 从数据库获取存储的 hash
            let row: Option<(String,)> =
                sqlx::query_as("SELECT value FROM settings WHERE key = 'admin_password'")
                    .fetch_optional(&state.db)
                    .await
                    .unwrap_or(None);

            let stored_hash = row.ok_or(AuthError::WrongCredentials)?.0;

            if security::verify_password(&payload.password, &stored_hash) {
                let token = security::create_jwt("admin", "all", None, &state.jwt_secret)?;
                return Ok(build_success_response("admin", "all", None, token));
            }
        }
        "vendor" => {
            // --- 摊主登录逻辑 ---

            // A. 先尝试全局 Admin 密码 (允许摊主用管理员密码登录)
            let admin_row: Option<(String,)> =
                sqlx::query_as("SELECT value FROM settings WHERE key = 'admin_password'")
                    .fetch_optional(&state.db)
                    .await
                    .unwrap_or(None);

            if let Some((hash,)) = admin_row {
                if security::verify_password(&payload.password, &hash) {
                    let token = security::create_jwt("vendor", "all", None, &state.jwt_secret)?;
                    return Ok(build_success_response("vendor", "all", None, token));
                }
            }

            // B. 尝试通用的 Vendor 密码 (如果在 settings 表里配置了的话)
            let vendor_row: Option<(String,)> =
                sqlx::query_as("SELECT value FROM settings WHERE key = 'vendor_password'")
                    .fetch_optional(&state.db)
                    .await
                    .unwrap_or(None);

            if let Some((hash,)) = vendor_row {
                if security::verify_password(&payload.password, &hash) {
                    let token = security::create_jwt("vendor", "all", None, &state.jwt_secret)?;
                    return Ok(build_success_response("vendor", "all", None, token));
                }
            }

            // C. 尝试特定 Event 的密码
            if let Some(eid) = event_id {
                // [修复] 验证 event 是否存在 ✓
                let event_exists: Option<(i64,)> =
                    sqlx::query_as("SELECT id FROM events WHERE id = ?")
                        .bind(eid)
                        .fetch_optional(&state.db)
                        .await
                        .unwrap_or(None);

                if event_exists.is_none() {
                    // Event 不存在，返回 404
                    return Err(AuthError::WrongCredentials);
                }

                let event_row: Option<(Option<String>,)> =
                    sqlx::query_as("SELECT vendor_password FROM events WHERE id = ?")
                        .bind(eid)
                        .fetch_optional(&state.db)
                        .await
                        .unwrap_or(None);

                // 注意：vendor_password 在数据库里是 nullable 的
                if let Some((Some(event_pass_hash),)) = event_row {
                    if security::verify_password(&payload.password, &event_pass_hash) {
                        let token =
                            security::create_jwt("vendor", "event", Some(eid), &state.jwt_secret)?;
                        return Ok(build_success_response("vendor", "event", Some(eid), token));
                    }
                }
            }
        }

        _ => return Err(AuthError::WrongCredentials),
    }

    Err(AuthError::WrongCredentials)
}

// 辅助函数：构建包含 Cookie 和 JSON Body 的响应
fn build_success_response(
    role: &str,
    access: &str,
    event_id: Option<i64>,
    token: String,
) -> Response {
    let body = LoginResponse {
        message: "Login successful".into(),
        role: role.into(),
        access: access.into(),
        event_id,
        token: token.clone(),
    };

    // 构建 Cookie 字符串 (HttpOnly + SameSite) [已修复]
    // SameSite=Lax: 允许顶级导航和安全跨域请求（GET）自动发送 Cookie，但POST/PUT/DELETE 不会
    // SameSite=Strict: 所有跨域请求都不会自动发送 Cookie
    let cookie_str = format!(
        "access_token_cookie={}; HttpOnly; Path=/; SameSite=Lax; Max-Age=86400",
        token
    );

    // 返回带有 Set-Cookie Header 的响应
    let mut response = Json(body).into_response();
    response.headers_mut().insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&cookie_str).unwrap(),
    );

    response
}
