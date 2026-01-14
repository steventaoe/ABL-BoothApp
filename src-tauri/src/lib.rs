use std::fs;
use std::path::PathBuf;
// [修改 1] DragDropEvent 现在直接在 tauri 模块下，WindowEvent 也建议引入
use tauri::{DragDropEvent, Emitter, Manager, WindowEvent};

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
        .plugin(tauri_plugin_os::init())
        //.plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_http::init())
        .invoke_handler(tauri::generate_handler![get_backend_url])
        .setup(|app| {
            // 1. 获取 AppHandle (Tauri v2 推荐方式)
            let app_handle = app.handle();

            // 调试：监听桌面端文件拖放事件，验证 webview 是否能收到 file-drop
            if let Some(main) = app_handle.get_webview_window("main") {
                // 调试：打印窗口事件，并在检测到文件拖放时向前端发送自定义事件
                let main_clone = main.clone();
                main.on_window_event(move |event| {
                    // 过滤掉频繁的事件，避免日志刷屏（可选）
                    // println!("[Debug][WindowEvent] {:?}", event);

                    if let WindowEvent::DragDrop(drop_event) = event {
                        // [修改 2] v2 中变体名称由 Dropped 改为 Drop
                        match drop_event {
                            DragDropEvent::Drop { paths, position } => {
                                println!(
                                    "[Debug][FileDrop][Backend] paths: {:?} @ {:?}",
                                    paths, position
                                );
                                // 将文件路径推送到前端
                                let _ = main_clone.emit("boothpack-file-drop", paths.clone());
                            }
                            // 处理其他拖拽状态（如 Enter, Over, Leave）以免编译警告
                            _ => {}
                        }
                    }
                });
            }

            // -------------------------------------------------------------
            // [优化点 1] 统一路径策略
            // -------------------------------------------------------------

            // A. 获取系统数据目录
            // 注意：需要在 tauri.conf.json 中配置 "identifier"，否则可能会报错
            let app_data_dir = app_handle
                .path()
                .app_data_dir()
                .expect("failed to get app data dir");

            // B. 上传文件目录
            let upload_dir = app_data_dir.join("uploads");

            println!("[Config] Database Path: {:?}", app_data_dir);
            println!("[Config] Uploads Path : {:?}", upload_dir);

            // 2. 确保目录存在
            if !upload_dir.exists() {
                fs::create_dir_all(&upload_dir).expect("Failed to create uploads directory");
            }

            // 3. 初始化数据库
            let db_pool = tauri::async_runtime::block_on(async {
                db::init_db(&app_data_dir)
                    .await
                    .expect("Database initialization failed")
            });

            // 4. 构建 AppState
            let state = state::AppState {
                db: db_pool,
                upload_dir: upload_dir.clone(),
                jwt_secret: std::env::var("JWT_SECRET")
                    .unwrap_or_else(|_| "secret_key_change_me".to_string()),
            };

            // 获取后端 URL
            let backend_url = std::env::var("BACKEND_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:5140".to_string());

            println!("[Config] Backend URL  : {}", backend_url);

            app.manage(backend_url.clone());

            // -------------------------------------------------------------
            // [优化点 2] 使用 Tauri 内置异步运行时
            // -------------------------------------------------------------
            tauri::async_runtime::spawn(async move {
                println!("[Booth Tool] Starting HTTP server on port 5140...");
                server::start_server(state, 5140).await;
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
