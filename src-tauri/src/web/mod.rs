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

pub async fn static_file_handler(uri: Uri, upload_dir: std::path::PathBuf) -> Response {
    let path = uri.path().trim_start_matches('/');

    // ----------------------------------------------------
    // 1. 优先处理物理文件 (上传的图片)
    // ----------------------------------------------------
    if path.starts_with("static/uploads/") {
        // 去掉 URL 前缀，拿到真实文件名
        let relative_path = path.trim_start_matches("static/uploads/");
        let file_path = upload_dir.join(relative_path);

        if file_path.exists() {
            // 使用 tower_http::services::ServeFile 或手动读取
            // 简单起见，这里手动读取二进制 (生产环境建议用 ServeDir)
            match tokio::fs::read(&file_path).await {
                Ok(bytes) => {
                    let mime = mime_guess::from_path(&file_path).first_or_octet_stream();
                    return (
                        [(header::CONTENT_TYPE, mime.as_ref())],
                        bytes,
                    ).into_response();
                },
                Err(_) => return (StatusCode::NOT_FOUND, "File access error").into_response(),
            }
        }
    }

    // ----------------------------------------------------
    // 2. 处理嵌入的 Vue 静态资源 (JS, CSS, Logo)
    // ----------------------------------------------------
    if let Some(content) = FrontendAssets::get(path) {
        let mime = mime_guess::from_path(path).first_or_octet_stream();
        return (
            [(header::CONTENT_TYPE, mime.as_ref())],
            content.data,
        ).into_response();
    }

    // ----------------------------------------------------
    // 3. SPA 路由回退 (Vue Router History 模式支持)
    // ----------------------------------------------------
    // 如果不是 API，不是图片，也不是存在的 JS/CSS 文件，
    // 统统返回 index.html，让 Vue Router 接管
    if let Some(index) = FrontendAssets::get("index.html") {
        return (
            [(header::CONTENT_TYPE, "text/html")],
            index.data,
        ).into_response();
    }

    (StatusCode::NOT_FOUND, "404 Not Found").into_response()
}