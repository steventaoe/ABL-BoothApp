use std::fs;
use std::path::PathBuf;
use tauri::Manager;

mod api;
mod db;
mod server;
mod state;
mod utils;
mod web;

// Tauri 命令：获取后端 URL
#[tauri::command]
fn get_backend_url(backend_url: tauri::State<String>) -> String {
    backend_url.inner().clone()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_backend_url])
        .setup(|app| {
            // 1. 获取 AppHandle (Tauri v2 推荐方式)
            let app_handle = app.handle();

            // -------------------------------------------------------------
            // [优化点 1] 统一路径策略
            // 数据库 (DB) 和上传文件 (Uploads) 都使用系统提供的 AppData 目录
            // 保证安全、持久化，并且所有平台行为一致
            // -------------------------------------------------------------

            // A. 获取系统数据目录 (所有平台统一)
            let app_data_dir = app_handle
                .path()
                .app_data_dir()
                .expect("failed to get app data dir");

            // B. 上传文件目录：统一使用 AppData/uploads
            let upload_dir = app_data_dir.join("uploads");

            // 打印路径方便调试
            println!("[Config] Database Path: {:?}", app_data_dir);
            println!("[Config] Uploads Path : {:?}", upload_dir);

            // 2. 确保目录存在
            if !upload_dir.exists() {
                fs::create_dir_all(&upload_dir).expect("Failed to create uploads directory");
            }

            // 3. 初始化数据库
            // 使用 tauri::async_runtime::block_on 在同步的 setup 钩子中执行异步代码
            // 传入 app_data_dir (确保数据库文件在安全的地方)
            let db_pool = tauri::async_runtime::block_on(async {
                db::init_db(&app_data_dir)
                    .await
                    .expect("Database initialization failed")
            });

            // 4. 构建 AppState
            let state = state::AppState {
                db: db_pool,
                upload_dir: upload_dir.clone(), // 传入我们自定义的 upload_dir
                // 生产环境建议生成随机密钥或从配置文件读取，这里用默认值兜底
                jwt_secret: std::env::var("JWT_SECRET")
                    .unwrap_or_else(|_| "secret_key_change_me".to_string()),
            };

            // 将后端服务 URL 注入到前端
            // 这样前端可以通过 window.__BACKEND_URL__ 访问，所有网络请求都走同一个地址
            let backend_url = std::env::var("BACKEND_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:5000".to_string());

            println!("[Config] Backend URL  : {}", backend_url);

            // 通过 Tauri 状态存储 URL，前端可以通过 invoke 命令获取
            app.manage(backend_url.clone());

            // -------------------------------------------------------------
            // [优化点 2] 使用 Tauri 内置异步运行时
            // -------------------------------------------------------------
            // 原代码使用了 std::thread::spawn + tokio::runtime::Runtime::new()
            // 缺点：创建了额外的线程和 Runtime 开销，且可能与 Tauri 的 Runtime 隔离。
            // 推荐：直接使用 tauri::async_runtime::spawn，它复用了 Tauri 的 Tokio Runtime。
            tauri::async_runtime::spawn(async move {
                println!("[Booth Tool] Starting HTTP server on port 5000...");
                server::start_server(state, 5000).await;
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
