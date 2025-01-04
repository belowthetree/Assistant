use std::fs;
use project_root::get_project_root;

#[tauri::command]
pub fn read_text_file(path: &str)->String {
    println!("读取 {}", path);
    let res = fs::read_to_string(path);
    if let Ok(s) = res {
        return s;
    }
    else {
        return "".into();
    }
}

#[tauri::command]
pub fn write_text_file(path: &str, content: &str) {
    println!("写入 {}", path);
    let res = fs::write(path, content);
    if let Err(e) = res {
        println!("写入失败 {}", e)
    }
}

#[tauri::command]
pub fn read_text_file_at_project_root(path: &str)->String {
    match get_project_root() {
        Ok(p) => {
            let root = p.to_str().unwrap().to_string();
            let s = root + "/../" + path;
            read_text_file(s.as_str())
        }
        Err(e) => {
            println!("获取根目录失败：{}", e);
            "".into()
        }
    }
}

#[tauri::command]
pub fn write_text_file_at_project_root(path: &str, content: &str) {
    match get_project_root() {
        Ok(p) => {
            let root = p.to_str().unwrap().to_string();
            let s = root + "/../" + path;
            write_text_file(&s, content);
        }
        Err(e) => {
            println!("获取根目录失败 {}", e)
        }
    }
}