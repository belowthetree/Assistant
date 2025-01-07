use project_root::get_project_root;
use std::{fs, path::Path};

#[tauri::command]
pub fn get_project_root_path() -> Result<String, String> {
    match get_project_root() {
        Ok(p) => {
            let root = p.to_str().unwrap().to_string();
            let t = fs::canonicalize(root + "/../");
            if let Ok(res) = t {
                Ok(res.to_str().unwrap().replace(r"\\?\", "").to_string())
            } else {
                Err(format!("{}", t.unwrap_err()))
            }
        }
        Err(e) => Err(format!("获取根目录失败：{}", e).into()),
    }
}

#[tauri::command]
pub fn read_text_file(path: &str) -> Result<String, String> {
    println!("读取 {}", path);
    let res = fs::read_to_string(path);
    if res.is_err() {
        println!("读取失败");
        Err(res.unwrap_err().to_string())
    } else {
        Ok(res.unwrap())
    }
}

#[tauri::command]
pub fn write_text_file(path: &str, content: &str) -> Result<(), String> {
    println!("写入 {}", path);
    if let Some(parent) = Path::new(path).parent() {
        if !Path::exists(parent) {
            println!("目录不存在，创建目录");
            if fs::create_dir_all(parent).is_err() {
                return Err(format!("创建 {} 上级目录失败", path));
            }
        }
    } else {
        return Err(format!("无法获取上级目录 {}", path));
    }
    let res = fs::write(path, content);
    if res.is_err() {
        Err(format!("写入失败 {}", res.unwrap_err()).into())
    } else {
        Ok(())
    }
}

#[tauri::command]
pub fn read_text_file_at_project_root(path: &str) -> Result<String, String> {
    match get_project_root() {
        Ok(p) => {
            let root = p.to_str().unwrap().to_string();
            let s = root + "/../" + path;
            read_text_file(s.as_str())
        }
        Err(e) => Err(format!("获取根目录失败：{}", e).into()),
    }
}

#[tauri::command]
pub fn write_text_file_at_project_root(path: &str, content: &str) -> Result<(), String> {
    match get_project_root() {
        Ok(p) => {
            let root = p.to_str().unwrap().to_string();
            let s = root + "/../" + path;
            write_text_file(&s, content)
        }
        Err(e) => Err(format!("获取根目录失败 {}", e).into()),
    }
}
