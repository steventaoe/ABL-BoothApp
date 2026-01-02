use axum::{
    extract::{Multipart, Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{query, query_as};

use crate::{
    api::guard::AdminOnly,
    db::models::Event,
    state::AppState,
    utils::{
        file::{delete_file, save_upload_file},
        security::hash_password,
    },
};

pub fn router() -> Router<AppState> {
    Router::new()
        // 公开接口
        .route("/", get(list_events))
        .route("/:id", get(get_event))
        // 管理员接口
        .route("/", post(create_event)) 
        .route("/:id", post(update_event).put(update_event)) 
        .route("/:id/status", put(update_status))
        .route("/:id", delete(delete_event))
}

// ==========================================
// DTOs (Data Transfer Objects)
// ==========================================

// 1. 用于查询参数解析
#[derive(Deserialize)]
struct ListEventsQuery {
    status: Option<String>,
}

// 2. 用于 API 响应的结构体 (解决 qrcode_url 问题)
#[derive(Serialize)]
struct EventResponse {
    #[serde(flatten)] // 将原始 Event 的字段展开 (id, name, date...)
    base: Event,
    
    // 覆盖/新增字段：对外暴露的 URL
    qrcode_url: Option<String>, 
}

impl EventResponse {
    // 转换函数：将 DB 模型转换为 API 响应模型
    fn from_model(event: Event) -> Self {
        // 假设静态文件服务挂载在 /static/ 上
        // 数据库存的是 "events/xxx.jpg"，转换成 "/static/uploads/events/xxx.jpg"
        let qrcode_url = event.payment_qr_code_path.as_ref().map(|path| {
            format!("/static/uploads/{}", path)
        });

        Self {
            base: event,
            qrcode_url,
        }
    }
}

// ==========================================
// 1. 获取漫展列表 (Public) [已修复过滤]
// ==========================================
async fn list_events(
    State(state): State<AppState>,
    Query(params): Query<ListEventsQuery>, // [修复] 接收 Query 参数
) -> impl IntoResponse {
    // 根据是否传了 status 决定 SQL
    let events: Vec<Event> = if let Some(status) = params.status {
        query_as::<_, Event>("SELECT * FROM events WHERE status = ? ORDER BY event_date DESC")
            .bind(status)
            .fetch_all(&state.db)
            .await
            .unwrap_or_default()
    } else {
        query_as::<_, Event>("SELECT * FROM events ORDER BY event_date DESC")
            .fetch_all(&state.db)
            .await
            .unwrap_or_default()
    };

    // [修复] 转换为包含 qrcode_url 的 Response 对象
    let response: Vec<EventResponse> = events.into_iter()
        .map(EventResponse::from_model)
        .collect();

    Json(response)
}

// ==========================================
// 2. 获取单个漫展 (Public) [已修复响应]
// ==========================================
async fn get_event(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let event: Option<Event> = query_as::<_, Event>("SELECT * FROM events WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .unwrap_or(None);

    match event {
        // [修复] 转换响应结构
        Some(e) => (StatusCode::OK, Json(EventResponse::from_model(e))).into_response(),
        None => (StatusCode::NOT_FOUND, Json(json!({"error": "Event not found"}))).into_response(),
    }
}

// ==========================================
// 3. 创建漫展 (Admin Only - Multipart)
// ==========================================
async fn create_event(
    State(state): State<AppState>,
    _: AdminOnly,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut name = String::new();
    let mut date = String::new();
    let mut location = String::new();
    let mut vendor_password = None;
    let mut qr_code_path: Option<String> = None;

    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        let field_name = field.name().unwrap_or("").to_string();

        if field_name == "payment_qr_code" {
            match save_upload_file(&state.upload_dir, field, Some("events")).await {
                Ok(path) => qr_code_path = Some(path),
                Err(e) => {
                    eprintln!("Upload Failed: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to save file"}))).into_response();
                }
            }
        } else {
            let value = field.text().await.unwrap_or_default();
            match field_name.as_str() {
                "name" => name = value,
                "date" => date = value,
                "location" => location = value,
                "vendor_password" => {
                    if !value.is_empty() {
                        vendor_password = Some(hash_password(&value));
                    }
                }
                _ => {}
            }
        }
    }

    if name.is_empty() || date.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(json!({"error": "Name and Date are required"}))).into_response();
    }

    let result = query(
        r#"
        INSERT INTO events (name, event_date, location, vendor_password, payment_qr_code_path, status)
        VALUES (?, ?, ?, ?, ?, '未进行')
        RETURNING id
        "#
    )
    .bind(name)
    .bind(date)
    .bind(if location.is_empty() { None } else { Some(location) })
    .bind(vendor_password)
    .bind(qr_code_path)
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(row) => {
            // 注意：Created 响应通常只返回简单信息或新对象，这里保持原样返回简单 JSON
            // 如果前端创建后立即需要 qrcode_url，这里也可以查一次库并返回 EventResponse
            use sqlx::Row;
            let id: i64 = row.try_get("id").unwrap_or(0);
            (StatusCode::CREATED, Json(json!({ "id": id, "message": "Event created" }))).into_response()
        }
        Err(e) => {
            eprintln!("DB Error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database Error").into_response()
        }
    }
}

// ==========================================
// 4. 更新漫展 (Admin Only - Multipart)
// ==========================================
async fn update_event(
    State(state): State<AppState>,
    _: AdminOnly,
    Path(id): Path<i64>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let old_event: Event = match query_as("SELECT * FROM events WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .unwrap_or(None) 
    {
        Some(e) => e,
        None => return (StatusCode::NOT_FOUND, "Event not found").into_response(),
    };

    let mut name = old_event.name;
    let mut date = old_event.event_date;
    let mut location = old_event.location;
    let mut vendor_password_hash = old_event.vendor_password;
    let mut qr_code_path = old_event.payment_qr_code_path;
    let mut should_remove_qr = false;

    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        let field_name = field.name().unwrap_or("").to_string();

        if field_name == "payment_qr_code" {
            match save_upload_file(&state.upload_dir, field, Some("events")).await {
                Ok(new_path) => {
                    if let Some(old_path) = &qr_code_path {
                         let _ = delete_file(&state.upload_dir, old_path).await;
                    }
                    qr_code_path = Some(new_path);
                }
                Err(e) => {
                    eprintln!("Update Upload Failed: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, "File upload failed").into_response();
                }
            }
        } else {
            let value = field.text().await.unwrap_or_default();
            match field_name.as_str() {
                "name" => name = value,
                "date" => date = value,
                "location" => location = if value.is_empty() { None } else { Some(value) },
                "vendor_password" => {
                    if !value.is_empty() {
                        vendor_password_hash = Some(hash_password(&value));
                    }
                },
                "remove_payment_qr_code" => {
                    if value == "true" {
                        should_remove_qr = true;
                    }
                }
                _ => {}
            }
        }
    }

    if should_remove_qr {
        if let Some(old_path) = &qr_code_path {
            let _ = delete_file(&state.upload_dir, old_path).await;
        }
        qr_code_path = None;
    }

    let result = query(
        r#"
        UPDATE events 
        SET name = ?, event_date = ?, location = ?, vendor_password = ?, payment_qr_code_path = ?
        WHERE id = ?
        "#
    )
    .bind(name)
    .bind(date)
    .bind(location)
    .bind(vendor_password_hash)
    .bind(qr_code_path)
    .bind(id)
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => (StatusCode::OK, Json(json!({"message": "Event updated"}))).into_response(),
        Err(e) => {
            eprintln!("Update DB Error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database Error").into_response()
        }
    }
}

// ==========================================
// 5. 更新状态 (Admin Only - JSON) [已修复状态验证]
// ==========================================
#[derive(Deserialize)]
struct UpdateStatusRequest {
    status: String,
}

async fn update_status(
    State(state): State<AppState>,
    _: AdminOnly,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateStatusRequest>,
) -> impl IntoResponse {
    // [修复] 验证状态值只能是允许的值 ✓
    match payload.status.as_str() {
        "未进行" | "进行中" | "已结束" => {
            // 状态值有效，继续处理
            let result = query("UPDATE events SET status = ? WHERE id = ?")
                .bind(&payload.status)
                .bind(id)
                .execute(&state.db)
                .await;

            match result {
                Ok(_) => (StatusCode::OK, Json(json!({"message": "Status updated"}))).into_response(),
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database Error").into_response(),
            }
        }
        _ => {
            // 无效的状态值
            (StatusCode::BAD_REQUEST, Json(json!({"error": "Invalid status. Must be one of: 未进行, 进行中, 已结束"}))).into_response()
        }
    }
}

// ==========================================
// 6. 删除漫展 (Admin Only) [已修复级联删除]
// ==========================================
async fn delete_event(
    State(state): State<AppState>,
    _: AdminOnly,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let event: Option<Event> = query_as("SELECT * FROM events WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .unwrap_or(None);

    if let Some(e) = event {
        // 1. 删除物理文件
        if let Some(path) = &e.payment_qr_code_path {
            let _ = delete_file(&state.upload_dir, &path).await;
        }
        
        // 2. [修复] 删除关联的 orders 和 products (级联删除) ✓
        // 注意：SQLite 需要开启 PRAGMA foreign_keys = ON 才能自动级联删除
        // 这里显式删除以确保不依赖 DB 配置
        
        // 先删除 orders 相关的 order_items
        let _ = sqlx::query(
            "DELETE FROM order_items WHERE order_id IN (SELECT id FROM orders WHERE event_id = ?)"
        )
        .bind(id)
        .execute(&state.db)
        .await;
        
        // 删除 orders
        let _ = sqlx::query("DELETE FROM orders WHERE event_id = ?")
            .bind(id)
            .execute(&state.db)
            .await;
        
        // 删除 products
        let _ = sqlx::query("DELETE FROM products WHERE event_id = ?")
            .bind(id)
            .execute(&state.db)
            .await;
        
        // 3. 最后删除 event
        let _ = query("DELETE FROM events WHERE id = ?")
            .bind(id)
            .execute(&state.db)
            .await;
            
        (StatusCode::OK, Json(json!({"message": "Event deleted"}))).into_response()
    } else {
        (StatusCode::NOT_FOUND, "Event not found").into_response()
    }
}