// src/api/info.rs

use crate::{state::AppState, utils::ip::get_lan_ip};
use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

pub fn router() -> Router<AppState> {
    Router::new().route("/server-info", get(server_info_handler))
}

async fn server_info_handler() -> Json<Value> {
    // 1. 获取动态的局域网 IP
    let ip = get_lan_ip();

    // 2. 端口配置 (这里硬编码了 5000，建议提取到常量或配置中)
    let port = 5000;

    // 3. 拼接 Base URL
    let base_url = format!("http://{}:{}", ip, port);

    // 4. 返回符合 API 文档的结构
    // 注意：这里的 URL 是前端页面的 URL (Vue Router 路径)，不是 API 路径
    Json(json!({
        "ip": ip,
        "port": port,
        "base_url": base_url,
        // 下面这几个 URL 是给二维码生成用的，指向前端页面
        "order_url": format!("{}/", base_url),   // 游客点单页
        "vendor_url": format!("{}/vendor", base_url),  // 摊主接单页
        "admin_url": format!("{}/admin", base_url),   // 管理员控制台
        // 同时也返回 API 的基础路径，方便前端调试
        "api_base_url": format!("{}/api", base_url)
    }))
}
