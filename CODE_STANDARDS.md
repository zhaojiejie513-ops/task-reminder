# 代码规范

本文档定义 task-reminder 项目的编码标准，适用于所有贡献者。

## 命名规范

### JavaScript（前端）

| 类型 | 风格 | 示例 |
|------|------|------|
| 变量/函数 | camelCase | `taskList`, `getTaskById` |
| 常量 | UPPER_SNAKE_CASE | `MAX_RETRY_COUNT` |
| CSS 类名 | kebab-case | `task-item`, `btn-primary` |
| 文件名 | kebab-case | `app.js`, `task-list.js` |

### Rust（后端）

| 类型 | 风格 | 示例 |
|------|------|------|
| 变量/函数 | snake_case | `task_list`, `get_task_by_id` |
| 结构体/枚举 | PascalCase | `TaskItem`, `AppState` |
| 常量 | UPPER_SNAKE_CASE | `MAX_CONNECTIONS` |
| 模块/文件 | snake_case | `commands.rs`, `db.rs` |

## 代码风格

### 通用

- 缩进：前端 2 空格，Rust 4 空格
- 行宽上限：100 字符
- 文件末尾保留一个空行
- 使用 LF 换行符

### JavaScript

- 使用 `const` / `let`，禁止 `var`
- 字符串使用单引号
- 语句末尾加分号
- 使用 `===` 而非 `==`
- `if/else/for/while` 必须使用花括号，即使只有一行

### Rust

- 使用 `?` 操作符传播错误，避免裸 `.unwrap()`
- 优先使用字段初始化简写（`Task { id, title }` 而非 `Task { id: id, title: title }`）
- 公开 API 添加文档注释（`///`）

## 注释规范

- 只在"为什么"不明显时写注释，不要注释"做了什么"
- 避免注释掉的代码留在仓库中
- TODO 格式：`// TODO: 描述`

## Git 提交规范

采用 Conventional Commits 格式：

```
<type>(<scope>): <description>

[可选正文]
```

**type 类型：**

- `feat` — 新功能
- `fix` — 修复 bug
- `docs` — 文档变更
- `style` — 格式调整（不影响逻辑）
- `refactor` — 重构
- `test` — 测试相关
- `chore` — 构建/工具变更

**示例：**

```
feat(tray): 添加右键菜单退出选项
fix(db): 修复任务完成状态未持久化的问题
docs: 更新 README 安装说明
```

## 工具链

| 工具 | 用途 | 命令 |
|------|------|------|
| ESLint | JS 静态检查 | `npm run lint` |
| Prettier | JS/CSS 格式化 | `npm run format` |
| rustfmt | Rust 格式化 | `cargo fmt` |
| Clippy | Rust 静态分析 | `cargo clippy` |

## 提交前检查

提交代码前请确保：

1. `npm run lint` 无错误
2. `npm run format:check` 通过
3. `cargo fmt --check`（在 src-tauri 目录）通过
4. `cargo clippy`（在 src-tauri 目录）无 error 级别警告
