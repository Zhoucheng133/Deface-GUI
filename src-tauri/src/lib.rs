use std::{process::Stdio, sync::Arc};
use tokio::io::{AsyncReadExt};
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

    let mut child = tokio::process::Command::new("deface")
        .args(args)
        .env("PYTHONUNBUFFERED", "1")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();
    
    *lock = Some(child);
    drop(lock);

    let window_stdout = window.clone();
    tokio::spawn(async move {
        let mut reader = BufReader::new(stdout);
        let mut buffer: [u8; 1024] = [0u8; 1024];
        loop {
            match reader.read(&mut buffer).await {
                Ok(0) => break,
                Ok(n) => {
                    let content = String::from_utf8_lossy(&buffer[..n]).to_string();
                    let _ = window_stdout.emit("log", content);
                }
                Err(_) => break,
            }
        }
    });

    let window_stderr = window.clone();
    tokio::spawn(async move {
        let mut reader = BufReader::new(stderr);
        let mut buffer: [u8; 1024] = [0u8; 1024];
        loop {
            match reader.read(&mut buffer).await {
                Ok(0) => break,
                Ok(n) => {
                    let content = String::from_utf8_lossy(&buffer[..n]).to_string();
                    let _ = window_stderr.emit("log", content);
                }
                Err(_) => break,
            }
        }
    });

    let state_clone = state.0.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            let mut lock = state_clone.lock().await;
            
            let is_finished = if let Some(c) = lock.as_mut() {
                match c.try_wait() {
                    Ok(Some(_status)) => true,
                    Ok(None) => false,
                    Err(_) => true,
                }
            } else {
                return;
            };

            if is_finished {
                *lock = None;
                let _ = window.emit("log", "✅ Done!");
                break;
            }
        }
    });

    Ok(())
}

#[tauri::command]
async fn stop_task(state: State<'_, CommandState>) -> Result<String, String> {
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
        .invoke_handler(tauri::generate_handler![run_task, stop_task])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
