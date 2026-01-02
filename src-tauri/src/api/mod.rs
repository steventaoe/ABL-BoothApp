use axum::Router;
use crate::state::AppState;

pub mod guard;
mod auth;
mod event;
mod master_product;
mod product;
mod order;
mod stats;
mod admin;
mod info;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/events", event::router()) // 包含 /events CRUD 和 /events/:id/status
        .nest("/events", stats::router()) // 包含 /events/:id/stats 等
        .nest("/master-products", master_product::router())
        .nest("/admin", admin::router()) // /api/admin/...
        .merge(info::router())           // /api/server-info
        .merge(product::router())        // /api/events/:id/products
        .merge(order::router())          // /api/events/:id/orders
}

