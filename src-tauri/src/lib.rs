use std::{process::Stdio, sync::Arc};
use tokio::io::AsyncBufReadExt;
use tokio::{io::BufReader, sync::Mutex};
use tokio::process::Child;
use tauri::{State, Window, Emitter};
pub struct CommandState(pub Arc<Mutex<Option<Child>>>);

#[tauri::command]
async fn run_task(
    window: Window, 
    state: State<'_, CommandState>, 
    args: Vec<String>
) -> Result<(), String> {
    let mut lock = state.0.lock().await;
    if lock.is_some() {
        return Err("已有任务在运行中".to_string());
    }
    let mut child=tokio::process::Command::new("deface")
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    let stdout = child.stdout.take().unwrap();
    *lock = Some(child);
    drop(lock);
    tokio::spawn(async move {
        let mut reader = BufReader::new(stdout).lines();
        while let Ok(Some(line)) = reader.next_line().await {
            let _ = window.emit("stdout", line);
        }
    });

    Ok(())
}

#[tauri::command]
async fn kill_task(state: State<'_, CommandState>) -> Result<String, String> {
    let mut lock = state.0.lock().await;

    if let Some(mut child) = lock.take() {
        #[cfg(target_os = "windows")]
        {
            if let Some(pid) = child.id() {
                let _ = std::process::Command::new("taskkill")
                    .args(["/F", "/T", "/PID", &pid.to_string()])
                    .spawn();
            }
        }

        let _ = child.kill().await;
        Ok("已停止当前任务".to_string())
    } else {
        Err("当前没有正在运行的任务".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(CommandState(Arc::new(Mutex::new(None))))
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![run_task, kill_task])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
