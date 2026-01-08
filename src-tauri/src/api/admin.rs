// src/api/admin.rs

use crate::{
    api::guard::AdminOnly,
    state::AppState,
    utils::security::{hash_password, verify_password},
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::put,
    Router,
};
use serde::Deserialize;
use serde_json::json;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/password", put(update_admin_password))
        .route(
            "/vendor-default-password",
            put(update_vendor_default_password),
        )
}

// ==========================================
// 1. 更新管理员密码
// ==========================================
#[derive(Deserialize)]
struct UpdateAdminPasswordRequest {
    #[serde(rename = "oldPassword")]
    old_password: String,
    #[serde(rename = "newPassword")]
    new_password: String,
}

async fn update_admin_password(
    State(state): State<AppState>,
    _: AdminOnly,
    Json(payload): Json<UpdateAdminPasswordRequest>,
) -> impl IntoResponse {
    // 1. 获取当前密码 Hash
    let row: Option<(String,)> =
        sqlx::query_as("SELECT value FROM settings WHERE key = 'admin_password'")
            .fetch_optional(&state.db)
            .await
            .unwrap_or(None);

    let stored_hash = match row {
        Some((h,)) => h,
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Admin password not set in database",
            )
                .into_response()
        }
    };

    // 2. 验证旧密码
    if !verify_password(&payload.old_password, &stored_hash) {
        eprintln!(
            "[DEBUG] Admin password verification failed. Stored hash: {}",
            stored_hash
        );
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "旧密码错误"})),
        )
            .into_response();
    }

    // 3. 更新新密码
    let new_hash = hash_password(&payload.new_password);
    eprintln!("[DEBUG] Updating admin password. New hash: {}", new_hash);
    let result = sqlx::query("UPDATE settings SET value = ? WHERE key = 'admin_password'")
        .bind(new_hash)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => {
            eprintln!("[DEBUG] Admin password updated successfully");
            (StatusCode::OK, Json(json!({"message": "管理员密码已更新"}))).into_response()
        }
        Err(e) => {
            eprintln!("[DEBUG] Failed to update admin password: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database Error").into_response()
        }
    }
}

// ==========================================
// 2. 更新默认摊主密码
// ==========================================
#[derive(Deserialize)]
struct UpdateVendorPasswordRequest {
    #[serde(rename = "newPassword")]
    new_password: String,
}

async fn update_vendor_default_password(
    State(state): State<AppState>,
    _: AdminOnly,
    Json(payload): Json<UpdateVendorPasswordRequest>,
) -> impl IntoResponse {
    let new_hash = hash_password(&payload.new_password);
    eprintln!(
        "[DEBUG] Updating global vendor password. New hash: {}",
        new_hash
    );

    // 使用 INSERT OR REPLACE 确保 key 存在
    let result =
        sqlx::query("INSERT OR REPLACE INTO settings (key, value) VALUES ('vendor_password', ?)")
            .bind(new_hash)
            .execute(&state.db)
            .await;

    match result {
        Ok(_) => {
            eprintln!("[DEBUG] Global vendor password updated successfully");
            (
                StatusCode::OK,
                Json(json!({"message": "默认摊主密码已更新"})),
            )
                .into_response()
        }
        Err(e) => {
            eprintln!("[DEBUG] Failed to update vendor password: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database Error").into_response()
        }
    }
}
