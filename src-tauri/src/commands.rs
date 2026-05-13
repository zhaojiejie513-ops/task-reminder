use tauri::State;
use crate::db::{Database, Task};

#[tauri::command]
pub fn get_tasks(db: State<'_, Database>) -> Vec<Task> {
    db.get_pending_tasks()
}

#[tauri::command]
pub fn get_completed_tasks(db: State<'_, Database>) -> Vec<Task> {
    db.get_completed_tasks()
}

#[tauri::command]
pub fn add_task(db: State<'_, Database>, content: String, priority: Option<i32>) -> Option<Task> {
    let content = content.trim();
    if content.is_empty() {
        return None;
    }
    Some(db.add_task(content, priority.unwrap_or(2)))
}

#[tauri::command]
pub fn update_task(db: State<'_, Database>, id: i64, content: Option<String>, priority: Option<i32>, due_at: Option<String>) -> bool {
    db.update_task(
        id,
        content.as_deref(),
        priority,
        Some(due_at.as_deref()),
    )
}

#[tauri::command]
pub fn complete_task(db: State<'_, Database>, id: i64) -> bool {
    db.complete_task(id)
}

#[tauri::command]
pub fn delete_completed_task(db: State<'_, Database>, id: i64) -> bool {
    db.delete_completed_task(id)
}

#[tauri::command]
pub fn restore_task(db: State<'_, Database>, id: i64) -> bool {
    db.restore_task(id)
}

#[tauri::command]
pub fn clear_completed_tasks(db: State<'_, Database>) -> usize {
    db.clear_completed_tasks()
}

#[tauri::command]
pub fn reorder_tasks(db: State<'_, Database>, ids: Vec<i64>) {
    db.reorder_tasks(&ids)
}

#[tauri::command]
pub fn get_pending_count(db: State<'_, Database>) -> usize {
    db.pending_count()
}
