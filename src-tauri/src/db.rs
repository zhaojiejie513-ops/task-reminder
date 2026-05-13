use rusqlite::{Connection, params};
use std::path::PathBuf;
use std::sync::Mutex;

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn new() -> Self {
        let db_path = Self::db_path();
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        let conn = Connection::open(&db_path).expect("Failed to open database");
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS tasks (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                content     TEXT NOT NULL,
                created_at  TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
                completed   INTEGER NOT NULL DEFAULT 0
            );"
        ).expect("Failed to create table");
        Self::migrate(&conn);
        Database { conn: Mutex::new(conn) }
    }

    fn migrate(conn: &Connection) {
        let version: i32 = conn
            .pragma_query_value(None, "user_version", |r| r.get(0))
            .unwrap_or(0);

        if version < 1 {
            conn.execute_batch("ALTER TABLE tasks ADD COLUMN completed_at TEXT;").ok();
            conn.execute_batch("ALTER TABLE tasks ADD COLUMN priority INTEGER NOT NULL DEFAULT 2;").ok();
            conn.execute_batch("ALTER TABLE tasks ADD COLUMN sort_order INTEGER NOT NULL DEFAULT 0;").ok();
            conn.execute_batch("ALTER TABLE tasks ADD COLUMN due_at TEXT;").ok();
            conn.execute_batch(
                "UPDATE tasks SET sort_order = id WHERE sort_order = 0 AND completed = 0;"
            ).ok();
            conn.pragma_update(None, "user_version", 1).ok();
        }
    }

    fn db_path() -> PathBuf {
        let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("task-reminder");
        path.push("tasks.db");
        path
    }

    pub fn get_pending_tasks(&self) -> Vec<Task> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT id, content, created_at, priority, sort_order, due_at FROM tasks WHERE completed = 0 ORDER BY priority DESC, sort_order ASC, created_at ASC")
            .unwrap();
        stmt.query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                content: row.get(1)?,
                created_at: row.get(2)?,
                priority: row.get(3)?,
                sort_order: row.get(4)?,
                due_at: row.get(5)?,
                completed_at: None,
            })
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .collect()
    }

    pub fn get_completed_tasks(&self) -> Vec<Task> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT id, content, created_at, priority, sort_order, due_at, completed_at FROM tasks WHERE completed = 1 ORDER BY completed_at DESC LIMIT 50")
            .unwrap();
        stmt.query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                content: row.get(1)?,
                created_at: row.get(2)?,
                priority: row.get(3)?,
                sort_order: row.get(4)?,
                due_at: row.get(5)?,
                completed_at: row.get(6)?,
            })
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .collect()
    }

    pub fn add_task(&self, content: &str, priority: i32) -> Task {
        let conn = self.conn.lock().unwrap();
        let next_order: i32 = conn
            .query_row("SELECT COALESCE(MAX(sort_order), 0) + 1 FROM tasks WHERE completed = 0", [], |row| row.get(0))
            .unwrap_or(1);
        conn.execute(
            "INSERT INTO tasks (content, priority, sort_order) VALUES (?1, ?2, ?3)",
            params![content, priority, next_order],
        ).unwrap();
        let id = conn.last_insert_rowid();
        let mut stmt = conn
            .prepare("SELECT id, content, created_at, priority, sort_order, due_at FROM tasks WHERE id = ?1")
            .unwrap();
        stmt.query_row(params![id], |row| {
            Ok(Task {
                id: row.get(0)?,
                content: row.get(1)?,
                created_at: row.get(2)?,
                priority: row.get(3)?,
                sort_order: row.get(4)?,
                due_at: row.get(5)?,
                completed_at: None,
            })
        })
        .unwrap()
    }

    pub fn update_task(&self, id: i64, content: Option<&str>, priority: Option<i32>, due_at: Option<Option<&str>>) -> bool {
        let conn = self.conn.lock().unwrap();
        let mut sets = Vec::new();
        let mut values: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(c) = content {
            sets.push("content = ?");
            values.push(Box::new(c.to_string()));
        }
        if let Some(p) = priority {
            sets.push("priority = ?");
            values.push(Box::new(p));
        }
        if let Some(d) = due_at {
            sets.push("due_at = ?");
            values.push(Box::new(d.map(|s| s.to_string())));
        }

        if sets.is_empty() {
            return false;
        }

        let sql = format!("UPDATE tasks SET {} WHERE id = ?", sets.join(", "));
        values.push(Box::new(id));

        let params: Vec<&dyn rusqlite::ToSql> = values.iter().map(|v| v.as_ref()).collect();
        conn.execute(&sql, params.as_slice()).unwrap_or(0) > 0
    }

    pub fn complete_task(&self, id: i64) -> bool {
        let conn = self.conn.lock().unwrap();
        let affected = conn
            .execute(
                "UPDATE tasks SET completed = 1, completed_at = datetime('now', 'localtime') WHERE id = ?1 AND completed = 0",
                params![id],
            )
            .unwrap_or(0);
        affected > 0
    }

    pub fn delete_completed_task(&self, id: i64) -> bool {
        let conn = self.conn.lock().unwrap();
        let affected = conn
            .execute("DELETE FROM tasks WHERE id = ?1 AND completed = 1", params![id])
            .unwrap_or(0);
        affected > 0
    }

    pub fn restore_task(&self, id: i64) -> bool {
        let conn = self.conn.lock().unwrap();
        let affected = conn
            .execute(
                "UPDATE tasks SET completed = 0, completed_at = NULL WHERE id = ?1 AND completed = 1",
                params![id],
            )
            .unwrap_or(0);
        affected > 0
    }

    pub fn clear_completed_tasks(&self) -> usize {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM tasks WHERE completed = 1", []).unwrap_or(0)
    }

    pub fn reorder_tasks(&self, ids: &[i64]) {
        let conn = self.conn.lock().unwrap();
        let tx = conn.unchecked_transaction().unwrap();
        for (i, id) in ids.iter().enumerate() {
            tx.execute("UPDATE tasks SET sort_order = ?1 WHERE id = ?2", params![i as i32 + 1, id]).ok();
        }
        tx.commit().ok();
    }

    pub fn get_overdue_tasks(&self) -> Vec<Task> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT id, content, created_at, priority, sort_order, due_at FROM tasks WHERE completed = 0 AND due_at IS NOT NULL AND due_at <= datetime('now', 'localtime')")
            .unwrap();
        stmt.query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                content: row.get(1)?,
                created_at: row.get(2)?,
                priority: row.get(3)?,
                sort_order: row.get(4)?,
                due_at: row.get(5)?,
                completed_at: None,
            })
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .collect()
    }

    pub fn pending_count(&self) -> usize {
        let conn = self.conn.lock().unwrap();
        conn.query_row("SELECT COUNT(*) FROM tasks WHERE completed = 0", [], |row| {
            row.get::<_, usize>(0)
        })
        .unwrap_or(0)
    }
}

#[derive(serde::Serialize, Clone)]
pub struct Task {
    pub id: i64,
    pub content: String,
    pub created_at: String,
    pub priority: i32,
    pub sort_order: i32,
    pub due_at: Option<String>,
    pub completed_at: Option<String>,
}
