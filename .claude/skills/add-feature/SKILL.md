---
name: add-feature
description: 为 task-reminder Tauri 应用添加新功能的完整工作流。当用户想要添加新功能、新增页面、新增按钮、新增数据库字段、新增 Tauri 命令时使用此 skill。即使用户只是说"加个功能"、"我想要一个XX按钮"、"能不能加个XX"，也应该触发这个 skill，因为它确保前后端和数据库的改动保持一致。
---

# Add Feature — 新功能开发工作流

本项目是一个 Tauri v2 桌面应用（Rust 后端 + 原生 JS 前端 + SQLite 数据库）。添加新功能通常需要同时修改多个层，这个 skill 确保你按正确的顺序完成，不遗漏任何一层。

## 项目结构速查

```
src/              → 前端（HTML/CSS/JS）
src-tauri/src/    → 后端（Rust）
  ├── main.rs    → 入口
  ├── lib.rs     → 应用初始化、插件注册
  ├── commands.rs → Tauri IPC 命令
  ├── db.rs      → 数据库操作
  └── tray.rs    → 系统托盘
```

## 开发顺序

添加功能时按这个顺序走，每一步完成后验证再进入下一步：

### 1. 数据层（如果需要新数据）

在 `src-tauri/src/db.rs` 中：
- 如果需要新表或新字段，在 `init_db()` 的建表语句中添加
- 添加对应的 CRUD 函数
- 数据结构用 `#[derive(Serialize, Deserialize)]` 标注，方便前后端传递

示例模式：
```rust
pub fn get_something(conn: &Connection) -> Result<Vec<Something>> {
    let mut stmt = conn.prepare("SELECT ... FROM ...")?;
    let items = stmt.query_map([], |row| {
        Ok(Something {
            id: row.get(0)?,
            // ...
        })
    })?.collect::<Result<Vec<_>, _>>()?;
    Ok(items)
}
```

### 2. 命令层

在 `src-tauri/src/commands.rs` 中：
- 添加 `#[tauri::command]` 函数
- 函数签名中用 `State` 获取数据库连接
- 返回 `Result<T, String>` 给前端

然后在 `src-tauri/src/lib.rs` 的 `invoke_handler` 中注册新命令。

### 3. 前端 UI

在 `src/` 中：
- `index.html` — 添加 HTML 结构
- `styles.css` — 添加样式
- `app.js` — 添加交互逻辑，通过 `window.__TAURI__.core.invoke('command_name', { args })` 调用后端

### 4. 验证

完成后执行：
```bash
npm run dev          # 启动开发模式，确认功能正常
npm run lint         # 前端代码检查
cargo clippy         # Rust 静态分析（在 src-tauri 目录）
```

## 注意事项

- 前后端通信通过 Tauri 的 `invoke` 机制，参数和返回值会自动序列化/反序列化
- 数据库改动要考虑向后兼容 — 如果用户已有旧数据库，新字段应该有默认值或用 `IF NOT EXISTS`
- 新命令必须在 `lib.rs` 的 `invoke_handler` 中注册，否则前端调用会报错
- CSS 类名用 kebab-case，JS 变量用 camelCase，Rust 用 snake_case
