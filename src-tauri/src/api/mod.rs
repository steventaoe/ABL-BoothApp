use crate::state::AppState;
use axum::Router;

mod admin;
mod auth;
mod event;
pub mod guard;
mod info;
mod master_product;
mod order;
mod product;
mod stats;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/events", event::router()) // 包含 /events CRUD 和 /events/:id/status
        .nest("/events", stats::router()) // 包含 /events/:id/stats 等
        .nest("/master-products", master_product::router())
        .nest("/admin", admin::router()) // /api/admin/...
        .merge(info::router()) // /api/server-info
        .merge(product::router()) // /api/events/:id/products
        .merge(order::router()) // /api/events/:id/orders
}
