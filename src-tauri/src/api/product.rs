use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, put},
    Router,
};
use serde::Deserialize;
use serde_json::json;
use sqlx::{query, query_as, Row};

use crate::{
    db::models::{MasterProduct, Product},
    state::AppState,
    utils::security::Claims,
};

pub fn router() -> Router<AppState> {
    // 注意路由结构：
    // 我们需要在 api/mod.rs 中把这个 router 挂载到根 /api 下，而不是 /api/products
    // 因为这里涉及两个不同的路径前缀：/events/... 和 /products/...
    Router::new()
        .route(
            "/events/:event_id/products",
            get(list_event_products).post(add_product_to_event),
        )
        .route(
            "/products/:product_id",
            put(update_product).delete(delete_product),
        )
}

// ==========================================
// 辅助：权限检查
// ==========================================
fn check_write_permission(
    claims: &Claims,
    target_event_id: i64,
) -> Result<(), (StatusCode, &'static str)> {
    if claims.role == "admin" {
        return Ok(());
    }
    if claims.role == "vendor" {
        if claims.access == "all" {
            return Ok(());
        }
        if let Some(eid) = claims.event_id {
            if eid == target_event_id {
                return Ok(());
            }
        }
    }
    Err((StatusCode::FORBIDDEN, "Permission denied for this event"))
}

// ==========================================
// 1. 获取场次库存列表 (Public)
// ==========================================
async fn list_event_products(
    State(state): State<AppState>,
    Path(event_id): Path<i64>,
) -> impl IntoResponse {
    // 关键点：JOIN master_products 获取图片和分类
    let sql = r#"
        SELECT 
            p.id, p.event_id, p.master_product_id, p.product_code, p.name, p.price, 
            p.initial_stock, p.current_stock, mp.image_url, mp.category
        FROM products p
        JOIN master_products mp ON p.master_product_id = mp.id
        WHERE p.event_id = ?
        ORDER BY p.product_code ASC
    "#;

    let products: Vec<Product> = query_as::<_, Product>(sql)
        .bind(event_id)
        .fetch_all(&state.db)
        .await
        .unwrap_or_default();

    Json(products)
}

// ==========================================
// 2. 添加商品到场次 (Admin/Vendor)
// ==========================================
#[derive(Deserialize)]
struct AddProductRequest {
    product_code: String,
    initial_stock: i64,
    price: Option<f64>, // 可选，不填则使用原价
}

async fn add_product_to_event(
    State(state): State<AppState>,
    claims: Claims, // 需要登录
    Path(event_id): Path<i64>,
    Json(payload): Json<AddProductRequest>,
) -> impl IntoResponse {
    // 1. 权限检查
    if let Err(e) = check_write_permission(&claims, event_id) {
        return e.into_response();
    }

    // [修复] 验证事件是否存在 ✓
    let event_exists: Option<(i64,)> = sqlx::query_as("SELECT id FROM events WHERE id = ?")
        .bind(event_id)
        .fetch_optional(&state.db)
        .await
        .unwrap_or(None);

    if event_exists.is_none() {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Event not found"})),
        )
            .into_response();
    }

    // 2. 查找全局商品信息 (Master Product)
    let master_prod: Option<MasterProduct> =
        query_as::<_, MasterProduct>("SELECT * FROM master_products WHERE product_code = ?")
            .bind(&payload.product_code)
            .fetch_optional(&state.db)
            .await
            .unwrap_or(None);

    let master = match master_prod {
        Some(mp) => mp,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Product code not found in master catalog"})),
            )
                .into_response()
        }
    };

    // 3. 确定价格 (如果 payload 没传，就用 default_price)
    let final_price = payload.price.unwrap_or(master.default_price);

    // 4. 插入库存表
    // 冗余存储 name 和 product_code 是为了快照，防止 master 删改后这里数据丢失
    let result = query(
        r#"
        INSERT INTO products 
        (event_id, master_product_id, product_code, name, price, initial_stock, current_stock)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(event_id)
    .bind(master.id)
    .bind(&master.product_code)
    .bind(&master.name)
    .bind(final_price)
    .bind(payload.initial_stock)
    .bind(payload.initial_stock) // 初始 current = initial
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => {
            // 获取最后插入的行ID
            let last_id: Option<(i64,)> = query_as("SELECT last_insert_rowid() as id")
                .fetch_optional(&state.db)
                .await
                .unwrap_or(None);

            if let Some((new_id,)) = last_id {
                // 构建Product对象（直接使用已有的master信息）
                let product = Product {
                    id: new_id,
                    event_id,
                    master_product_id: master.id,
                    product_code: master.product_code.clone(),
                    name: master.name.clone(),
                    price: final_price,
                    initial_stock: payload.initial_stock,
                    current_stock: payload.initial_stock,
                    image_url: master.image_url.clone(),
                    category: master.category.clone(),
                };

                (StatusCode::CREATED, Json(product)).into_response()
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get insert ID").into_response()
            }
        }
        Err(e) => {
            // 检查是否重复添加
            if e.to_string().contains("UNIQUE constraint") {
                (
                    StatusCode::CONFLICT,
                    Json(json!({"error": "Product already in this event"})),
                )
                    .into_response()
            } else {
                eprintln!("Insert product error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database Error").into_response()
            }
        }
    }
}

// ==========================================
// 3. 更新库存/价格 (Admin/Vendor)
// ==========================================
#[derive(Deserialize)]
struct UpdateProductRequest {
    price: Option<f64>,
    initial_stock: Option<i64>,
}

async fn update_product(
    State(state): State<AppState>,
    claims: Claims,
    Path(product_id): Path<i64>,
    Json(payload): Json<UpdateProductRequest>,
) -> impl IntoResponse {
    // 1. 先查这个 Product 属于哪个 Event，以便校验权限
    let product: Option<Product> = query_as("SELECT * FROM products WHERE id = ?")
        .bind(product_id)
        .fetch_optional(&state.db)
        .await
        .unwrap_or(None);

    let product = match product {
        Some(p) => p,
        None => return (StatusCode::NOT_FOUND, "Product not found").into_response(),
    };

    // 2. 权限检查
    if let Err(e) = check_write_permission(&claims, product.event_id) {
        return e.into_response();
    }

    // 3. 计算新库存
    // 逻辑：current_stock 应该跟随 initial_stock 的变化而变化 (保持已售数量不变)
    // sold = old_initial - old_current
    // new_current = new_initial - sold
    // [修复] 防止库存变成负数 ✓
    let mut new_initial = product.initial_stock;
    let mut new_current = product.current_stock;
    let mut new_price = product.price;

    if let Some(p) = payload.price {
        new_price = p;
    }

    if let Some(init) = payload.initial_stock {
        let sold = product.initial_stock - product.current_stock;
        new_initial = init;
        new_current = init - sold;

        // [修复] 如果 sold > new_init，意味着试图将初始库存设置为小于已售数量
        // 这是非法的，应该返回错误而不是允许负数库存
        if new_current < 0 {
            return (StatusCode::BAD_REQUEST, Json(json!({
                "error": format!(
                    "Cannot set initial stock to {} (already sold {}). New initial must be at least {}",
                    init, sold, sold
                )
            }))).into_response();
        }
    }

    // 4. 更新数据库
    let result =
        query("UPDATE products SET price = ?, initial_stock = ?, current_stock = ? WHERE id = ?")
            .bind(new_price)
            .bind(new_initial)
            .bind(new_current)
            .bind(product_id)
            .execute(&state.db)
            .await;

    match result {
        Ok(_) => {
            // 返回更新后的对象（使用内连接，master_product必定存在）
            let updated_product = query_as::<_, Product>(
                r#"
                SELECT p.id, p.event_id, p.master_product_id, p.product_code, p.name, p.price, 
                       p.initial_stock, p.current_stock, mp.image_url, mp.category 
                FROM products p 
                JOIN master_products mp ON p.master_product_id = mp.id 
                WHERE p.id = ?
                "#,
            )
            .bind(product_id)
            .fetch_one(&state.db)
            .await
            .unwrap();

            (StatusCode::OK, Json(updated_product)).into_response()
        }
        Err(e) => {
            eprintln!("Update product error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database Error").into_response()
        }
    }
}

// ==========================================
// 4. 删除商品 (Admin/Vendor)
// ==========================================
async fn delete_product(
    State(state): State<AppState>,
    claims: Claims,
    Path(product_id): Path<i64>,
) -> impl IntoResponse {
    // 1. 查 Event ID
    let row: Option<(i64,)> = sqlx::query_as("SELECT event_id FROM products WHERE id = ?")
        .bind(product_id)
        .fetch_optional(&state.db)
        .await
        .unwrap_or(None);

    let event_id = match row {
        Some((id,)) => id,
        None => return (StatusCode::NOT_FOUND, "Product not found").into_response(),
    };

    // 2. 权限检查
    if let Err(e) = check_write_permission(&claims, event_id) {
        return e.into_response();
    }

    // 3. 删除
    let _ = query("DELETE FROM products WHERE id = ?")
        .bind(product_id)
        .execute(&state.db)
        .await;

    (
        StatusCode::OK,
        Json(json!({"message": "Product removed from event"})),
    )
        .into_response()
}
