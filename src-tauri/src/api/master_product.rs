use axum::{
    extract::{Multipart, Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post, put}, // 注意：axum 中 update multipart 通常用 post 兼容性更好
    Router,
};
use serde::Deserialize;
use serde_json::json;
use sqlx::{query, query_as};

use crate::{
    api::guard::AdminOnly,
    db::models::MasterProduct,
    state::AppState,
    utils::file::{delete_file, save_upload_file},
};

pub fn router() -> Router<AppState> {
    Router::new()
        // 公开接口
        .route("/", get(list_products))
        // 管理员接口
        .route("/", post(create_product))
        .route("/:id", post(update_product).put(update_product)) // 更新商品详情 (含图片)
        .route("/:id/status", put(update_status)) // 上下架
}

// ==========================================
// 1. 获取商品列表 (Public)
// ==========================================
#[derive(Deserialize)]
struct ListQuery {
    all: Option<bool>, // ?all=true 显示所有，否则只显示 is_active=true
}

async fn list_products(
    State(state): State<AppState>,
    Query(params): Query<ListQuery>,
) -> impl IntoResponse {
    let show_all = params.all.unwrap_or(false);

    let sql = if show_all {
        "SELECT * FROM master_products ORDER BY product_code ASC"
    } else {
        "SELECT * FROM master_products WHERE is_active = 1 ORDER BY product_code ASC"
    };

    let products: Vec<MasterProduct> = query_as::<_, MasterProduct>(sql)
        .fetch_all(&state.db)
        .await
        .unwrap_or_default();

    Json(products)
}

// ==========================================
// 2. 创建商品 (Admin Only - Multipart)
// ==========================================
async fn create_product(
    State(state): State<AppState>,
    _: AdminOnly,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut product_code = String::new();
    let mut name = String::new();
    let mut default_price: f64 = 0.0;
    let mut category: Option<String> = None;
    let mut image_path: Option<String> = None;

    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        let field_name = field.name().unwrap_or("").to_string();

        if field_name == "image" {
            // [文件处理] 存入 products 子目录
            match save_upload_file(&state.upload_dir, field, Some("products")).await {
                Ok(path) => image_path = Some(path),
                Err(e) => {
                    eprintln!("Upload error: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, "File upload failed").into_response();
                }
            }
        } else {
            let value = field.text().await.unwrap_or_default();
            match field_name.as_str() {
                "product_code" => product_code = value,
                "name" => name = value,
                "default_price" => {
                    // [数值解析] 字符串转浮点
                    default_price = value.parse().unwrap_or(0.0);
                }
                "category" => category = if value.is_empty() { None } else { Some(value) },
                _ => {}
            }
        }
    }

    // 必填项检查
    if product_code.is_empty() || name.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(json!({"error": "Code and Name are required"}))).into_response();
    }

    // 执行插入
    let result = query(
        r#"
        INSERT INTO master_products (product_code, name, default_price, category, image_url)
        VALUES (?, ?, ?, ?, ?)
        RETURNING id
        "#
    )
    .bind(product_code)
    .bind(name)
    .bind(default_price)
    .bind(category)
    .bind(image_path)
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(row) => {
            use sqlx::Row;
            let id: i64 = row.try_get("id").unwrap_or(0);
            (StatusCode::CREATED, Json(json!({ "id": id, "message": "Product created" }))).into_response()
        }
        Err(e) => {
            // [错误处理] 检查是否是唯一性约束冲突 (SQLite error code 2067 or 1555)
            // SQLx 的错误处理比较底层，这里简单判断一下
            let error_msg = e.to_string();
            if error_msg.contains("UNIQUE constraint failed") {
                (StatusCode::CONFLICT, Json(json!({"error": "Product code already exists"}))).into_response()
            } else {
                eprintln!("DB Error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database Error").into_response()
            }
        }
    }
}

// ==========================================
// 3. 更新商品 (Admin Only - Multipart)
// ==========================================
async fn update_product(
    State(state): State<AppState>,
    _: AdminOnly,
    Path(id): Path<i64>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    // 1. 查旧数据
    let old_product: MasterProduct = match query_as("SELECT * FROM master_products WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .unwrap_or(None)
    {
        Some(p) => p,
        None => return (StatusCode::NOT_FOUND, "Product not found").into_response(),
    };

    // 2. 准备更新容器
    let mut product_code = old_product.product_code;
    let mut name = old_product.name;
    let mut default_price = old_product.default_price;
    let mut category = old_product.category;
    let mut image_path = old_product.image_url;
    let mut should_remove_image = false;

    // 3. 解析 Multipart
    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        let field_name = field.name().unwrap_or("").to_string();

        if field_name == "image" {
            // 上传新图
            match save_upload_file(&state.upload_dir, field, Some("products")).await {
                Ok(new_path) => {
                    // 标记旧图删除
                    if let Some(old_path) = &image_path {
                        let _ = delete_file(&state.upload_dir, old_path).await;
                    }
                    image_path = Some(new_path);
                }
                Err(e) => eprintln!("Update upload error: {}", e),
            }
        } else {
            let value = field.text().await.unwrap_or_default();
            match field_name.as_str() {
                "product_code" => product_code = value,
                "name" => name = value,
                "default_price" => {
                    if !value.is_empty() {
                         default_price = value.parse().unwrap_or(default_price);
                    }
                },
                "category" => category = if value.is_empty() { None } else { Some(value) },
                "remove_image" => {
                    if value == "true" {
                        should_remove_image = true;
                    }
                }
                _ => {}
            }
        }
    }

    // 4. 处理显式删除图片
    if should_remove_image {
        if let Some(old_path) = &image_path {
            let _ = delete_file(&state.upload_dir, old_path).await;
        }
        image_path = None;
    }

    // 5. 数据库更新
    let result = query(
        r#"
        UPDATE master_products 
        SET product_code = ?, name = ?, default_price = ?, category = ?, image_url = ?
        WHERE id = ?
        "#
    )
    .bind(product_code)
    .bind(name)
    .bind(default_price)
    .bind(category)
    .bind(image_path)
    .bind(id)
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => (StatusCode::OK, Json(json!({"message": "Product updated"}))).into_response(),
        Err(e) => {
             // 同样也要检查唯一性冲突（如果修改了 product_code）
             let error_msg = e.to_string();
             if error_msg.contains("UNIQUE constraint failed") {
                 (StatusCode::CONFLICT, Json(json!({"error": "Product code already exists"}))).into_response()
             } else {
                 (StatusCode::INTERNAL_SERVER_ERROR, "Database Error").into_response()
             }
        }
    }
}

// ==========================================
// 4. 更新上下架状态 (Admin Only - JSON)
// ==========================================
#[derive(Deserialize)]
struct UpdateStatusRequest {
    is_active: bool,
}

async fn update_status(
    State(state): State<AppState>,
    _: AdminOnly,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateStatusRequest>,
) -> impl IntoResponse {
    let result = query("UPDATE master_products SET is_active = ? WHERE id = ?")
        .bind(payload.is_active)
        .bind(id)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => (StatusCode::OK, Json(json!({"message": "Status updated"}))).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database Error").into_response(),
    }
}