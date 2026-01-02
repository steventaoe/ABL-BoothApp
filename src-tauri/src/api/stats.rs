use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json, Response},
    routing::get,
    Router,
};
use rust_xlsxwriter::Workbook;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{
    state::AppState,
    utils::security::Claims,
    db::models::Event,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/:event_id/stats", get(get_event_stats))
        .route("/:event_id/sales_summary", get(get_sales_summary))
        .route("/:event_id/sales_summary/download", get(download_sales_summary))
}

// ==========================================
// DTOs
// ==========================================

#[derive(Serialize)]
struct StatsResponse {
    event_info: Event,
    summary: SummaryStats,
    product_details: Vec<ProductStat>,
}

#[derive(Serialize, FromRow, Default)]
struct SummaryStats {
    total_revenue: f64,
    completed_orders_count: i64,
    total_items_sold: i64,
}

#[derive(Serialize, FromRow)]
struct ProductStat {
    product_id: i64,
    name: String,
    sold_count: i64,
    revenue: f64,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct SummaryQuery {
    product_code: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    interval_minutes: Option<i64>, // 30 or 60
}

// ==========================================
// 1. 获取仪表盘统计 (Dashboard Stats)
// ==========================================
async fn get_event_stats(
    State(state): State<AppState>,
    claims: Claims,
    Path(event_id): Path<i64>,
) -> impl IntoResponse {
    if let Err(e) = check_read_permission(&claims, event_id) {
        return e.into_response();
    }

    // 1. 获取场次信息
    let event = match sqlx::query_as::<_, Event>("SELECT * FROM events WHERE id = ?")
        .bind(event_id)
        .fetch_optional(&state.db)
        .await
        .unwrap_or(None) 
    {
        Some(e) => e,
        None => return (StatusCode::NOT_FOUND, "Event not found").into_response(),
    };

    // 2. 获取汇总数据 (只统计 completed 订单)
    let summary: SummaryStats = sqlx::query_as(
        r#"
        SELECT 
            COALESCE(SUM(o.total_amount), 0.0) as total_revenue,
            COUNT(o.id) as completed_orders_count,
            COALESCE(SUM(oi.quantity), 0) as total_items_sold
        FROM orders o
        LEFT JOIN order_items oi ON o.id = oi.order_id
        WHERE o.event_id = ? AND o.status = 'completed'
        "#
    )
    .bind(event_id)
    .fetch_one(&state.db)
    .await
    .unwrap_or_default();

    // 3. 获取商品销售详情
    let product_details = sqlx::query_as::<_, ProductStat>(
        r#"
        SELECT 
            oi.product_id,
            oi.product_name as name,
            SUM(oi.quantity) as sold_count,
            SUM(oi.product_price * oi.quantity) as revenue
        FROM order_items oi
        JOIN orders o ON oi.order_id = o.id
        WHERE o.event_id = ? AND o.status = 'completed'
        GROUP BY oi.product_id, oi.product_name
        ORDER BY revenue DESC
        "#
    )
    .bind(event_id)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    Json(StatsResponse {
        event_info: event,
        summary,
        product_details,
    }).into_response()
}

// ==========================================
// 2. 获取销售趋势图数据 (Sales Summary Chart)
// ==========================================
async fn get_sales_summary(
    State(state): State<AppState>,
    claims: Claims,
    Path(event_id): Path<i64>,
    Query(_params): Query<SummaryQuery>,
) -> impl IntoResponse {
    if let Err(e) = check_read_permission(&claims, event_id) {
        return e.into_response();
    }

    // 这里为了简化演示，我们返回按时间排序的原始订单数据，
    // 或者你可以使用 SQLite 的 strftime 进行分组。
    // 鉴于移动端 SQLite 版本差异，我们在 Rust 层做分组可能更灵活，
    // 但为了性能，这里演示简单的 SQL 时间分组 (按小时)。
    
    #[derive(Serialize, FromRow)]
    struct TimeSeriesPoint {
        time_bucket: String,
        revenue: f64,
        order_count: i64,
    }

    // SQLite 时间格式化: %Y-%m-%d %H:00
    let sql = r#"
        SELECT 
            strftime('%Y-%m-%d %H:00', created_at) as time_bucket,
            SUM(total_amount) as revenue,
            COUNT(id) as order_count
        FROM orders
        WHERE event_id = ? AND status = 'completed'
        GROUP BY time_bucket
        ORDER BY time_bucket ASC
    "#;

    let points = sqlx::query_as::<_, TimeSeriesPoint>(sql)
        .bind(event_id)
        .fetch_all(&state.db)
        .await
        .unwrap_or_default();

    Json(points).into_response()
}

// ==========================================
// 3. 导出 Excel (Excel Download)
// ==========================================
async fn download_sales_summary(
    State(state): State<AppState>,
    claims: Claims,
    Path(event_id): Path<i64>,
) -> Response {
    if let Err(e) = check_read_permission(&claims, event_id) {
        return e.into_response();
    }

    // 1. 获取数据 (复用 product_details 的逻辑，但可能需要更详细的数据)
    // 这里我们导出一份详细的商品销售表
    let details = sqlx::query_as::<_, ProductStat>(
        r#"
        SELECT 
            oi.product_id,
            oi.product_name as name,
            SUM(oi.quantity) as sold_count,
            SUM(oi.product_price * oi.quantity) as revenue
        FROM order_items oi
        JOIN orders o ON oi.order_id = o.id
        WHERE o.event_id = ? AND o.status = 'completed'
        GROUP BY oi.product_id, oi.product_name
        ORDER BY revenue DESC
        "#
    )
    .bind(event_id)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    // 2. 创建 Excel 工作簿到临时文件
    use std::fs;
    use axum::http::header::{HeaderMap, HeaderValue};
    
    let temp_dir = std::env::temp_dir();
    let filename = format!("sales_report_event_{}.xlsx", event_id);
    let temp_file = temp_dir.join(&filename);

    let mut workbook = Workbook::new(temp_file.to_str().unwrap_or("report.xlsx"));
    let worksheet = workbook.add_worksheet();

    // 创建默认格式对象（无特殊格式）
    let default_format = rust_xlsxwriter::Format::new();

    // 写表头
    let _ = worksheet.write_string(0, 0, "商品ID", &default_format);
    let _ = worksheet.write_string(0, 1, "商品名称", &default_format);
    let _ = worksheet.write_string(0, 2, "销售数量", &default_format);
    let _ = worksheet.write_string(0, 3, "销售总额", &default_format);

    // 写数据
    for (i, item) in details.iter().enumerate() {
        let row = (i + 1) as u32;
        let _ = worksheet.write_number(row, 0, item.product_id as f64, &default_format);
        let _ = worksheet.write_string(row, 1, &item.name, &default_format);
        let _ = worksheet.write_number(row, 2, item.sold_count as f64, &default_format);
        let _ = worksheet.write_number(row, 3, item.revenue, &default_format);
    }

    // 3. 完成写入并读取为 Buffer
    drop(worksheet);
    match workbook.close() {
        Ok(_) => {
            // 读取文件内容
            match fs::read(&temp_file) {
                Ok(buf) => {
                    // 删除临时文件
                    let _ = fs::remove_file(&temp_file);
                    
                    // 构建响应
                    let mut headers = HeaderMap::new();
                    headers.insert(
                        axum::http::header::CONTENT_TYPE,
                        HeaderValue::from_static("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"),
                    );
                    headers.insert(
                        axum::http::header::CONTENT_DISPOSITION,
                        HeaderValue::from_str(&format!("attachment; filename=\"sales_report_event_{}.xlsx\"", event_id))
                            .unwrap_or_else(|_| HeaderValue::from_static("attachment")),
                    );
                    
                    (headers, buf).into_response()
                },
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read generated excel file").into_response(),
            }
        },
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to generate excel").into_response(),
    }
}

// 权限辅助函数
fn check_read_permission(claims: &Claims, event_id: i64) -> Result<(), (StatusCode, &'static str)> {
    if claims.role == "admin" {
        return Ok(());
    }
    if claims.role == "vendor" {
        if claims.access == "all" || claims.event_id == Some(event_id) {
            return Ok(());
        }
    }
    Err((StatusCode::FORBIDDEN, "Access denied"))
}