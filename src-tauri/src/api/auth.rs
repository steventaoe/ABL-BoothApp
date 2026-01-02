use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
    Router, routing::post,
    http::{header, HeaderValue},
};
use serde::{Deserialize, Serialize};

use crate::{state::AppState, utils::security::{self, AuthError}};

// 1. 请求体 DTO
#[derive(Deserialize)]
struct LoginRequest {
    role: String,           // "admin" | "vendor"
    password: String,
    event_id: Option<i64>,  // 驼峰转下划线需注意，这里假设前端传的是 snake_case 或者配置了 serde
    #[serde(rename = "eventId")] // 兼容前端传过来的驼峰命名
    event_id_camel: Option<i64>, 
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
    Json(payload): Json<LoginRequest>,
) -> Result<Response, AuthError> {
    
    // 统一 event_id (前端可能传 eventId 或 event_id)
    let event_id = payload.event_id_camel.or(payload.event_id);

    match payload.role.as_str() {
        "admin" => {
            // --- 管理员登录逻辑 ---
            // 从数据库获取存储的 hash
            let row: Option<(String,)> = sqlx::query_as("SELECT value FROM settings WHERE key = 'admin_password'")
                .fetch_optional(&state.db)
                .await
                .unwrap_or(None);

            let stored_hash = row.ok_or(AuthError::WrongCredentials)?.0;

            if security::verify_password(&payload.password, &stored_hash) {
                let token = security::create_jwt("admin", "all", None, &state.jwt_secret)?;
                return Ok(build_success_response("admin", "all", None, token));
            }
        },
        "vendor" => {
            // --- 摊主登录逻辑 ---
            
            // A. 先尝试全局 Admin 密码 (允许摊主用管理员密码登录)
            let admin_row: Option<(String,)> = sqlx::query_as("SELECT value FROM settings WHERE key = 'admin_password'")
                .fetch_optional(&state.db)
                .await.unwrap_or(None);
            
            if let Some((hash,)) = admin_row {
                if security::verify_password(&payload.password, &hash) {
                    let token = security::create_jwt("vendor", "all", None, &state.jwt_secret)?;
                    return Ok(build_success_response("vendor", "all", None, token));
                }
            }

            // B. 尝试通用的 Vendor 密码 (如果在 settings 表里配置了的话)
             let vendor_row: Option<(String,)> = sqlx::query_as("SELECT value FROM settings WHERE key = 'vendor_password'")
                .fetch_optional(&state.db)
                .await.unwrap_or(None);
            
            if let Some((hash,)) = vendor_row {
                if security::verify_password(&payload.password, &hash) {
                     let token = security::create_jwt("vendor", "all", None, &state.jwt_secret)?;
                    return Ok(build_success_response("vendor", "all", None, token));
                }
            }

            // C. 尝试特定 Event 的密码
            if let Some(eid) = event_id {
                // [修复] 验证 event 是否存在 ✓
                let event_exists: Option<(i64,)> = sqlx::query_as("SELECT id FROM events WHERE id = ?")
                    .bind(eid)
                    .fetch_optional(&state.db)
                    .await
                    .unwrap_or(None);

                if event_exists.is_none() {
                    // Event 不存在，返回 404
                    return Err(AuthError::WrongCredentials); // 或者可以自定义一个特殊错误类型返回 404
                }

                let event_row: Option<(Option<String>,)> = sqlx::query_as("SELECT vendor_password FROM events WHERE id = ?")
                    .bind(eid)
                    .fetch_optional(&state.db)
                    .await
                    .unwrap_or(None);

                // 注意：vendor_password 在数据库里是 nullable 的
                if let Some((Some(event_pass_hash),)) = event_row {
                    if security::verify_password(&payload.password, &event_pass_hash) {
                         let token = security::create_jwt("vendor", "event", Some(eid), &state.jwt_secret)?;
                        return Ok(build_success_response("vendor", "event", Some(eid), token));
                    }
                }
            }
        },
        _ => return Err(AuthError::WrongCredentials),
    }

    Err(AuthError::WrongCredentials)
}

// 辅助函数：构建包含 Cookie 和 JSON Body 的响应
fn build_success_response(role: &str, access: &str, event_id: Option<i64>, token: String) -> Response {
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