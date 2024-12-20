use std::process::Command;
use walkdir::WalkDir;
use lnk_parser::{self, LNKParser};

#[tauri::command]
pub fn exec_cmd(arg: &str) -> Result<String, String> {
    let status;
    if cfg!(target_os = "windows") {
        let newcmd = format!("{}", arg);
        status = Command::new("cmd").args(&["/c", &newcmd]).spawn()
    } else {
        println!("不支持的系统");
        return Err("不支持的系统".into());
    }
    if status.is_ok() {
        println!("Application opened successfully");
    } else {
        eprintln!("Failed to open application");
    }
    Ok("命令执行成功".into())
}

#[tauri::command]
pub fn open_app_by_shortcut(arg: &str)-> Result<String, String> {
    let status;
    let path = get_path_by_app_name(arg);
    if path.is_err() {
        return Err(path.unwrap_err());
    }
    if cfg!(target_os = "windows") {
        let res_path: String;
        if arg.ends_with(".lnk") {
            let lnk = LNKParser::from_path(path.unwrap().as_str());
            if lnk.is_err() {
                println!("lnk 获取失败 {:?}", lnk);
                return Err("".into());
            }
            res_path = lnk.unwrap().get_target_full_path().clone().unwrap();
        } else {
            res_path = path.unwrap();
        }
        println!("path {}", res_path);
        let newcmd = format!("{}", res_path);
        status = Command::new("cmd").args(&["/c", &newcmd]).spawn()
    } else {
        println!("不支持的系统");
        return Err("不支持的系统".into());
    }
    if status.is_ok() {
        println!("Application opened successfully");
    } else {
        eprintln!("Failed to open application");
    }
    Ok("open app 命令执行成功".into())
}

#[tauri::command]
pub fn open_app_by_name(arg: &str)-> Result<String, String> {
    let status;
    if cfg!(target_os = "windows") {
        let newcmd = format!("start {}", arg);
        status = Command::new("cmd").args(&["/c", &newcmd]).spawn()
    } else {
        println!("不支持的系统");
        return Err("不支持的系统".into());
    }
    if status.is_ok() {
        println!("打开程序成功");
    } else {
        eprintln!("打开程序失败");
    }
    Ok("open_app_by_shortcut 命令执行成功".into())
}

#[tauri::command]
pub fn get_all_app_names()->String {
    if cfg!(target_os = "windows") {
        let mut  res: Vec<String> = Vec::new();
        for entry in WalkDir::new("C:/Users/ZGG/AppData/Roaming/Microsoft/Windows/Start Menu/Programs")
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok()) {
                if entry.metadata().is_ok() && entry.metadata().unwrap().is_file() {
                    res.push(entry.file_name().to_str().unwrap().into());
                }
        }
        serde_json::to_string(&res).unwrap()
    } else {
        println!("不支持的系统");
        "不支持的系统".into()
    }
}

pub fn get_path_by_app_name(arg: &str)->Result<String, String> {
    if cfg!(target_os = "windows") {
        for entry in WalkDir::new("C:/Users/ZGG/AppData/Roaming/Microsoft/Windows/Start Menu/Programs")
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok()) {
                if entry.metadata().is_ok() && entry.metadata().unwrap().is_file() {
                    if entry.file_name().to_str().unwrap() == arg {
                        return Ok(entry.path().to_str().unwrap().into());
                    }
                }
        }
        Err(arg.to_string() + "未找到 app 文件")
    } else {
        println!("不支持的系统");
        Err("不支持的系统".into())
    }
}