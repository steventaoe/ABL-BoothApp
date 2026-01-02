use tauri::Manager;
use std::fs;
use std::path::PathBuf;

mod state;
mod db;
mod server;
mod api;
mod utils;
mod web;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // 1. 获取 AppHandle (Tauri v2 推荐方式)
            let app_handle = app.handle();

            // -------------------------------------------------------------
            // [优化点 1] 路径分离策略
            // 数据库 (DB): 始终建议放在 AppData (系统管理的沙盒/数据区)，保证安全和持久化。
            // 图片 (Uploads): 根据你的需求，Windows 放运行目录，Android 放沙盒。
            // -------------------------------------------------------------
            
            // A. 获取数据库路径 (所有平台统一使用 AppData)
            let app_data_dir = app_handle.path().app_data_dir().expect("failed to get app data dir");
            
            // B. 获取上传路径 (条件编译)
            let upload_dir: PathBuf;

            #[cfg(mobile)]
            {
                // Android/iOS: 必须用沙盒路径，否则无权限
                upload_dir = app_data_dir.join("uploads");
            }

            #[cfg(not(mobile))]
            {
                // Windows/macOS: 使用当前工作目录 (开发时是 src-tauri 目录)
                // 这样你可以直接在文件夹里看到上传的图片
                upload_dir = std::env::current_dir().expect("failed to get current dir").join("uploads");
            }

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
                db::init_db(&app_data_dir).await.expect("Database initialization failed")
            });

            // 4. 构建 AppState
            let state = state::AppState {
                db: db_pool,
                upload_dir: upload_dir.clone(), // 传入我们自定义的 upload_dir
                // 生产环境建议生成随机密钥或从配置文件读取，这里用默认值兜底
                jwt_secret: std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret_key_change_me".to_string()),
            };

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