// src/utils/file.rs

use axum::extract::multipart::Field;
use std::path::PathBuf;
use tokio::fs;
use uuid::Uuid;

/// 保存上传的文件
///
/// - `base_dir`: 物理根目录 (state.upload_dir)
/// - `field`: Axum 的 Multipart Field
/// - `sub_folder`: (可选) 子文件夹，比如 "events" 或 "products"，方便分类
///
/// 返回: Result<可直接访问的完整路径 (包含 /uploads/ 前缀), 错误信息>
pub async fn save_upload_file(
    base_dir: &PathBuf,
    field: Field<'_>,
    sub_folder: Option<&str>,
) -> Result<String, String> {
    // 1. 获取并净化文件名
    let file_name = field.file_name().unwrap_or("unknown").to_string();

    // 2. 获取扩展名 (默认为 jpg)
    let ext = std::path::Path::new(&file_name)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("jpg"); // 这里可以加一个白名单检查，防止上传 .exe 等危险文件

    // 3. 生成唯一文件名 (UUID)
    let new_filename = format!("{}.{}", Uuid::new_v4(), ext);

    // 4. 确定存储子目录 (例如: uploads/products/)
    let mut file_path = base_dir.clone();
    let mut relative_path_str = String::new();

    if let Some(folder) = sub_folder {
        file_path.push(folder);
        // 确保子目录存在
        if !file_path.exists() {
            fs::create_dir_all(&file_path)
                .await
                .map_err(|e| e.to_string())?;
        }
        relative_path_str.push_str(folder);
        relative_path_str.push('/');
    }

    // 拼接最终路径
    file_path.push(&new_filename);
    relative_path_str.push_str(&new_filename);

    // 5. 读取并写入文件
    // TODO: 如果未来需要图片压缩 (image crate)，可以在这里拦截 bytes 进行处理
    let data = field.bytes().await.map_err(|e| e.to_string())?;

    fs::write(&file_path, data)
        .await
        .map_err(|e| format!("Write failed: {}", e))?;

    // 返回完整的绝对路径，例如 "/uploads/products/550e8400...jpg"
    // 前端在任何路由下都能正确加载：<img :src="image_url" />
    Ok(format!("/uploads/{}", relative_path_str))
}

/// 删除文件
///
/// - `relative_path`: 完整路径，如 "uploads/products/xxx.jpg"
pub async fn delete_file(base_dir: &PathBuf, relative_path: &str) -> std::io::Result<()> {
    if relative_path.is_empty() {
        return Ok(());
    }

    // 防御性编程：防止路径回溯攻击 (../)
    if relative_path.contains("..") {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid path",
        ));
    }

    // 去掉 "uploads/" 前缀（如果存在）
    let path_without_uploads = relative_path.trim_start_matches("uploads/");

    let file_path = base_dir.join(path_without_uploads);
    if file_path.exists() {
        fs::remove_file(file_path).await?;
    }
    Ok(())
}
