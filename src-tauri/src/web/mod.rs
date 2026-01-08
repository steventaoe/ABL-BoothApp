use axum::{
    body::Body,
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
};
use rust_embed::RustEmbed;

// ========================================================
// 核心：指向 Vue 的构建产物目录
// 路径含义：从 src-tauri 目录向上一级(..)，进入 frontend/dist
// ========================================================
#[derive(RustEmbed)]
#[folder = "../frontend/dist/"]
struct FrontendAssets;

pub async fn static_file_handler(uri: Uri, _upload_dir: std::path::PathBuf) -> Response {
    let path = uri.path().trim_start_matches('/');

    // ----------------------------------------------------
    // 注意：上传文件的物理访问已在 server.rs 的 fallback 中处理
    // 这里只负责处理嵌入的 Vue 静态资源
    // ----------------------------------------------------

    // ----------------------------------------------------
    // 1. 处理嵌入的 Vue 静态资源 (JS, CSS, Logo)
    // ----------------------------------------------------
    if let Some(content) = FrontendAssets::get(path) {
        let mime = mime_guess::from_path(path).first_or_octet_stream();
        return ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response();
    }

    // ----------------------------------------------------
    // 2. SPA 路由回退 (Vue Router History 模式支持)
    // ----------------------------------------------------
    // 如果不是 API，不是图片，也不是存在的 JS/CSS 文件，
    // 统统返回 index.html，让 Vue Router 接管
    if let Some(index) = FrontendAssets::get("index.html") {
        return ([(header::CONTENT_TYPE, "text/html")], index.data).into_response();
    }

    (StatusCode::NOT_FOUND, "404 Not Found").into_response()
}
