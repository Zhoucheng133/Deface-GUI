use std::sync::{Arc, Mutex}; 
use tauri::menu::{Menu, PredefinedMenuItem, Submenu};
use tauri::{State, Window, Emitter, AppHandle};
use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::{CommandChild, CommandEvent};

pub struct CommandState(pub Arc<Mutex<Option<CommandChild>>>);

#[tauri::command]
fn check_path(dir: String, file: String) -> bool {
    return std::path::Path::new(&dir).join(&file).exists()
}

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
        .env("PYTHONUNBUFFERED", "1")
        .env("PYTHONUTF8", "1");

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
                            "\n✅ 任务完成" 
                        } else { 
                            "\n❌ 任务失败" 
                        };
                        let _ = window.emit("end", msg);
                    } else {
                        let _ = window.emit("end", "\n🛑 任务已终止");
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
        let _ = child.kill();
        Ok("已发送停止指令".to_string())
    } else {
        Err("当前没有正在运行的任务".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            let locale = "zh";
            let (app_menu_name, quit_label, edit_label, copy_label, paste_label, select_all_label, undo_label, redo_label, window_label, minimize_label, fullscreen_label, about_label, hide_label) = 
            if locale == "zh" {
                ("应用", "退出 Deface GUI", "编辑", "复制", "粘贴", "全选", "撤销", "重做", "窗口", "最小化", "进入全屏幕", "关于 Deface GUI", "隐藏 Deface GUI")
            } else {
                ("App", "Quit Deface GUI", "Edit", "Copy", "Paste", "Select All", "Undo", "Redo", "Window", "Minimize", "Fullscreen", "About Deface GUI", "Hide Deface GUI")
            };
            
            let app_menu = Submenu::with_items(
                handle, 
                app_menu_name, 
                true, 
                &[
                    &PredefinedMenuItem::about(handle, Some(about_label), None)?,
                    &PredefinedMenuItem::separator(handle)?,
                    &PredefinedMenuItem::hide(handle, Some(hide_label))?,
                    &PredefinedMenuItem::quit(handle, Some(quit_label))?
                ]
            )?;

            let edit_menu = Submenu::with_items(
                handle,
                edit_label,
                true,
                &[
                    &PredefinedMenuItem::undo(handle, Some(undo_label))?,
                    &PredefinedMenuItem::redo(handle, Some(redo_label))?,
                    &PredefinedMenuItem::separator(handle)?,
                    &PredefinedMenuItem::copy(handle, Some(copy_label))?,
                    &PredefinedMenuItem::paste(handle, Some(paste_label))?,
                    &PredefinedMenuItem::select_all(handle, Some(select_all_label))?,
                ],
            )?;

            let window_menu = Submenu::with_items(
                handle,
                window_label,
                true,
                &[
                    &PredefinedMenuItem::minimize(handle, Some(minimize_label))?,
                    &PredefinedMenuItem::fullscreen(handle, Some(fullscreen_label))?,
                ],
            )?;

            let menu = Menu::with_items(handle, &[&app_menu, &edit_menu, &window_menu])?;
            app.set_menu(menu)?;
            Ok(())
        })
        .manage(CommandState(Arc::new(Mutex::new(None))))
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![run_task, stop_task, check_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
