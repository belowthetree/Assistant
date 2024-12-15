use std::process::Command;

#[tauri::command]
pub fn open_app(name: &str) -> Result<String, String> {
    let status;
    if cfg!(target_os = "windows") {
        let cmd = format!("{}", name);
        status = Command::new("cmd").args(&["/c", &cmd]).spawn()
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
