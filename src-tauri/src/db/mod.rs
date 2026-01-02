pub mod models;

use sqlx::{sqlite::SqlitePoolOptions, Sqlite, SqlitePool};
use std::path::PathBuf;
use std::fs;
use crate::utils::security::hash_password;

pub async fn init_db(app_data_dir: &PathBuf) -> Result<SqlitePool, sqlx::Error> {
    use sqlx::migrate::MigrateDatabase;
    
    // 1. 确保数据库文件路径存在
    if !app_data_dir.exists() {
        fs::create_dir_all(app_data_dir).expect("Failed to create app data dir");
    }
    
    // 2. 拼接数据库文件路径: /.../data/sale_system.db
    let db_path = app_data_dir.join("sale_system.db");
    let db_url = format!("sqlite://{}", db_path.to_string_lossy());

    // 3. 如果文件不存在，创建它
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        Sqlite::create_database(&db_url).await?;
    }

    // 4. 连接
    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await?;

    // 5. 运行迁移 (使用运行时方式避免编译时需要 DATABASE_URL)
    sqlx::migrate!().run(&pool).await?;

    // ================== 新增逻辑 ==================
    // 6. 检查并初始化默认管理员密码
    // 假设默认密码是 "admin123" (生产环境应强制用户首次登录修改，这里为了演示简化)
    let admin_exists: (i64,) = sqlx::query_as("SELECT count(*) FROM settings WHERE key = 'admin_password'")
        .fetch_one(&pool)
        .await?;

    if admin_exists.0 == 0 {
        let password_hash = hash_password("admin123");
        sqlx::query("INSERT INTO settings (key, value) VALUES ('admin_password', ?)")
            .bind(password_hash)
            .execute(&pool)
            .await?;
        println!("Initialized default admin password.");
    }

    Ok(pool)
}
