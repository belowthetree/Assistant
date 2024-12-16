use std::process::Command;

#[tauri::command]
pub fn exec_cmd(cmd: &str) -> Result<String, String> {
    let status;
    if cfg!(target_os = "windows") {
        let newcmd = format!("{}", cmd);
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
