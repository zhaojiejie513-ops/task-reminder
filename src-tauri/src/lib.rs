mod commands;
mod db;
mod tray;

use std::collections::HashSet;
use std::sync::Mutex;
use tauri::Manager;
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--autostart"]),
        ))
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            let autostart = app.autolaunch();
            let _ = autostart.enable();

            let db = db::Database::new();
            let has_tasks = db.pending_count() > 0;
            app.manage(db);

            tray::setup_tray(app.handle())?;

            let is_autostart = std::env::args().any(|a| a == "--autostart");

            if let Some(window) = app.get_webview_window("main") {
                if is_autostart && !has_tasks {
                    window.hide().ok();
                } else if has_tasks {
                    window.set_always_on_top(true).ok();
                    let win = window.clone();
                    std::thread::spawn(move || {
                        std::thread::sleep(std::time::Duration::from_secs(3));
                        win.set_always_on_top(false).ok();
                    });
                }

                let win = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        win.hide().ok();
                    }
                });
            }

            // Reminder thread: check overdue tasks every 60 seconds
            let handle = app.handle().clone();
            std::thread::spawn(move || {
                let notified: Mutex<HashSet<i64>> = Mutex::new(HashSet::new());
                loop {
                    std::thread::sleep(std::time::Duration::from_secs(60));
                    let db = handle.state::<db::Database>();
                    let overdue = db.get_overdue_tasks();
                    let mut set = notified.lock().unwrap();
                    for task in &overdue {
                        if set.contains(&task.id) {
                            continue;
                        }
                        set.insert(task.id);
                        use tauri_plugin_notification::NotificationExt;
                        handle.notification()
                            .builder()
                            .title("任务已到期")
                            .body(&task.content)
                            .show()
                            .ok();
                    }
                    // Remove IDs no longer overdue (completed or deleted)
                    let overdue_ids: HashSet<i64> = overdue.iter().map(|t| t.id).collect();
                    set.retain(|id| overdue_ids.contains(id));
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_tasks,
            commands::get_completed_tasks,
            commands::add_task,
            commands::update_task,
            commands::complete_task,
            commands::delete_completed_task,
            commands::restore_task,
            commands::clear_completed_tasks,
            commands::reorder_tasks,
            commands::get_pending_count,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
