use axum::{
    body::Body,
    extract::Request,
    http::{header, HeaderValue, Method, StatusCode, Uri},
    response::{IntoResponse, Response},
    Router,
};
use std::net::SocketAddr;
use std::path::PathBuf;
use tower::ServiceBuilder;
use tower_http::cors::{AllowHeaders, AllowOrigin, CorsLayer};

use crate::{api, state::AppState, web};

pub async fn start_server(state: AppState, port: u16) {
    // 复制一份 upload_dir 供 fallback 闭包使用
    let upload_dir = state.upload_dir.clone();

    // 打印一下当前的上传根目录，确保它符合预期
    println!("[Server Debug] Upload Root Path: {:?}", upload_dir);

    let app = Router::new()
        .nest("/api", api::router()) // API 路由
        .fallback(move |req: Request| {
            let uri = req.uri().clone();
            let upload_dir = upload_dir.clone(); // 再次 clone 进入 async 块

            async move {
                let path = uri.path();

                // ---------------------------------------------------------
                // 1. 处理静态资源图片请求
                // ---------------------------------------------------------
                if path.starts_with("/static/uploads/") {
                    // 关键步骤：去掉 URL 前缀 "/static/uploads/"
                    // 变成 "products/xxx.jpg"
                    let relative_path = path.trim_start_matches("/static/uploads/");

                    // 解码 URL (处理文件名中的空格、中文等)
                    // 需要在 Cargo.toml 添加 urlencoding 依赖，或者暂时忽略
                    let decoded_path = urlencoding::decode(relative_path)
                        .unwrap_or(std::borrow::Cow::Borrowed(relative_path));

                    // 拼接物理路径
                    let file_path = upload_dir.join(decoded_path.as_ref());

                    // [调试日志] 打印出来看看它到底在找哪
                    println!("[Static] Req: {} -> Looking at: {:?}", path, file_path);

                    if file_path.exists() && file_path.is_file() {
                        match tokio::fs::read(&file_path).await {
                            Ok(bytes) => {
                                // 自动猜测 MIME 类型 (image/jpeg, image/png)
                                let mime = mime_guess::from_path(&file_path).first_or_octet_stream();
                                return (
                                    [(header::CONTENT_TYPE, mime.as_ref())],
                                    bytes
                                ).into_response();
                            },
                            Err(e) => {
                                println!("[Static Error] Read failed: {}", e);
                                return (StatusCode::INTERNAL_SERVER_ERROR, "File Read Error").into_response();
                            }
                        }
                    } else {
                        println!("[Static Error] File not found: {:?}", file_path);
                        return (StatusCode::NOT_FOUND, "Image Not Found").into_response();
                    }
                }

                // ---------------------------------------------------------
                // 2. 处理前端 Vue 页面 (嵌入资源)
                // ---------------------------------------------------------
                web::static_file_handler(uri, upload_dir).await
            }
        })
        .layer(
            ServiceBuilder::new().layer(
                CorsLayer::new()
                    .allow_origin(AllowOrigin::mirror_request())
                    .allow_methods([
                        Method::GET, Method::POST, Method::PUT, 
                        Method::DELETE, Method::PATCH, Method::OPTIONS
                    ])
                    .allow_headers(AllowHeaders::list(vec![
                        header::AUTHORIZATION,
                        header::CONTENT_TYPE,
                        header::COOKIE,
                        header::HeaderName::from_static("x-requested-with"),
                    ]))
                    .allow_credentials(true),
            ),
        )
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("[Booth Tool] HTTP Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind server address");

    axum::serve(listener, app).await.expect("Server crashed");
}