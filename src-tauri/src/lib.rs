use std::sync::{Arc, Mutex}; 
use tauri::{State, Window, Emitter, AppHandle};
use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::{CommandChild, CommandEvent};

pub struct CommandState(pub Arc<Mutex<Option<CommandChild>>>);

#[tauri::command]
async fn run_task(
    app: AppHandle,
    window: Window, 
    state: State<'_, CommandState>, 
    args: Vec<String>
) -> Result<(), String> {
    let mut lock = state.0.lock().map_err(|_| "锁获取失败")?;
    
    if lock.is_some() {
        return Err("已有任务在运行中".to_string());
    }

    let cmd = app.shell().command("deface")
        .args(args)
        .env("PYTHONUNBUFFERED", "1");

    let (mut rx, child) = cmd
        .spawn()
        .map_err(|e| e.to_string())?;

    *lock = Some(child);
    drop(lock); 

    let state_clone = state.0.clone();
    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(line_bytes) => {
                    let content = String::from_utf8_lossy(&line_bytes).to_string();
                    let _ = window.emit("log", content);
                }
                CommandEvent::Stderr(line_bytes) => {
                    let content = String::from_utf8_lossy(&line_bytes).to_string();
                    let _ = window.emit("log", content);
                }
                CommandEvent::Terminated(payload) => {
                    let mut lock = state_clone.lock().unwrap();
                    *lock = None;

                    if let Some(code) = payload.code {
                        let msg = if code == 0 {
                            "\n✅ 任务成功完成！" 
                        } else { 
                            "\n❌ 任务失败" 
                        };
                        let _ = window.emit("log", msg);
                    } else {
                        let _ = window.emit("log", "\n🛑 任务已终止");
                    }
                }
                _ => {}
            }
        }
    });

    Ok(())
}

#[tauri::command]
async fn stop_task(state: State<'_, CommandState>) -> Result<String, String> {
    // 获取标准库锁
    let mut lock = state.0.lock().map_err(|_| "锁获取失败")?;

    if let Some(child) = lock.take() {
        #[cfg(target_os = "windows")]
        {
            let pid = child.pid();
            let _ = std::process::Command::new("taskkill")
                .args(["/F", "/T", "/PID", &pid.to_string()])
                .creation_flags(0x08000000)
                .spawn();
        }

        let _ = child.kill();
        Ok("已发送停止指令".to_string())
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
