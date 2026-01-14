use axum::{
    body::Body,
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
};
use rust_embed::RustEmbed;
use std::env;

// ========================================================
// 核心：指向 Vue 的构建产物目录
// ========================================================
#[derive(RustEmbed)]
#[folder = "$FRONTEND_DIST_ABS_PATH"]
struct FrontendAssets;

pub async fn static_file_handler(uri: Uri, _upload_dir: std::path::PathBuf) -> Response {
    let path = uri.path().trim_start_matches('/');

    println!("--------------------------------------------------");
    println!(">>> 收到请求路径: '{}'", path);

    // 1. 打印当前环境诊断信息 (只在请求 index.html 或根路径时打印，避免刷屏)
    if path == "" || path == "index.html" {
        println!(">>> [环境诊断] 正在运行诊断...");
        
        // A. 打印手机上的当前工作目录 (CWD)
        // 这能让你明白为什么手机上找物理文件会失败
        match env::current_dir() {
            Ok(cwd) => println!(">>> [环境诊断] 手机当前工作目录: {:?}", cwd),
            Err(e) => println!(">>> [环境诊断] 无法获取工作目录: {}", e),
        }

        // B. 遍历 RustEmbed 里的所有文件
        // 这是最关键的一步：查看二进制文件肚子里到底有没有货
        println!(">>> [资源清单] 开始列出 RustEmbed 内部包含的所有文件:");
        let mut count = 0;
        for file in FrontendAssets::iter() {
            println!("   - 已打包文件: {}", file.as_ref());
            count += 1;
        }
        if count == 0 {
            println!(">>> [严重警告] RustEmbed 列表为空！编译时没有找到任何文件！");
            println!(">>> [可能原因] 编译路径 '../frontend/dist/' 解析错误，或该目录下确实为空。");
        } else {
            println!(">>> [资源清单] 共找到 {} 个嵌入文件。", count);
        }
        println!("--------------------------------------------------");
    }

    // ----------------------------------------------------
    // 2. 正常处理逻辑
    // ----------------------------------------------------
    if let Some(content) = FrontendAssets::get(path) {
        println!(">>> [成功] 返回静态资源: {}", path);
        let mime = mime_guess::from_path(path).first_or_octet_stream();
        return ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response();
    }

    // SPA 回退逻辑
    if let Some(index) = FrontendAssets::get("index.html") {
        println!(">>> [回退] 路径 '{}' 未找到，回退到 index.html", path);
        return ([(header::CONTENT_TYPE, "text/html")], index.data).into_response();
    }

    println!(">>> [失败] 彻底放弃治疗。未找到 '{}' 且未找到 'index.html'", path);
    (StatusCode::NOT_FOUND, "404 Not Found").into_response()
}