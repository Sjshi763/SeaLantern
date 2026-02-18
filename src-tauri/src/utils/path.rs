use std::path::PathBuf;

/// 获取应用程序数据目录
/// 
/// 根据不同平台返回合适的存储路径：
/// - Windows: %AppData%\Sea Lantern
/// - macOS: ~/Library/Application Support/Sea Lantern
/// - Linux: ~/.config/sea-lantern
/// 
/// 这个函数确保 MSI 安装的应用将数据存储在用户目录而非安装目录
pub fn get_app_data_dir() -> PathBuf {
    // 优先使用系统提供的标准数据目录
    if let Some(data_dir) = dirs_next::data_dir() {
        #[cfg(target_os = "windows")]
        {
            // Windows: %AppData%\Sea Lantern
            return data_dir.join("Sea Lantern");
        }
        
        #[cfg(target_os = "macos")]
        {
            // macOS: ~/Library/Application Support/Sea Lantern
            return data_dir.join("Sea Lantern");
        }
        
        #[cfg(target_os = "linux")]
        {
            // Linux: ~/.local/share/sea-lantern
            return data_dir.join("sea-lantern");
        }
    }
    
    // 如果无法获取标准数据目录，回退到主目录
    if let Some(home_dir) = dirs_next::home_dir() {
        #[cfg(target_os = "windows")]
        {
            return home_dir.join(".sea-lantern");
        }
        
        #[cfg(target_os = "macos")]
        {
            return home_dir.join("Library").join("Application Support").join("Sea Lantern");
        }
        
        #[cfg(target_os = "linux")]
        {
            return home_dir.join(".sea-lantern");
        }
    }
    
    // 最后的回退方案：使用当前目录
    PathBuf::from(".")
}

/// 获取应用数据目录的字符串表示，如果目录不存在则创建
pub fn get_or_create_app_data_dir() -> String {
    let data_dir = get_app_data_dir();
    
    // 创建目录（如果不存在）
    if let Err(e) = std::fs::create_dir_all(&data_dir) {
        eprintln!("警告：无法创建数据目录：{}", e);
    }
    
    data_dir.to_string_lossy().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_app_data_dir_not_empty() {
        let dir = get_app_data_dir();
        assert!(!dir.as_path().as_os_str().is_empty());
    }
    
    #[test]
    fn test_get_or_create_app_data_dir() {
        let dir_str = get_or_create_app_data_dir();
        assert!(!dir_str.is_empty());
        
        // 验证目录存在
        let path = PathBuf::from(&dir_str);
        assert!(path.exists());
        assert!(path.is_dir());
    }
}
