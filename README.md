# Task Reminder

一款轻量级桌面待办事项应用，基于 Tauri 2 + Vue 3 构建，支持任务优先级、截止时间提醒、拖拽排序等功能。

![Windows](https://img.shields.io/badge/platform-Windows-blue)
![Tauri](https://img.shields.io/badge/Tauri-2.x-orange)
![Vue](https://img.shields.io/badge/Vue-3.x-green)

## 功能特性

- 📝 快速创建/编辑/删除任务
- 🔥⭐🌱 三级优先级标记（紧急、普通、轻松）
- 📅 截止时间设置与到期提醒通知
- 🔀 拖拽排序，自由调整任务顺序
- ✅ 完成任务归档，支持恢复已完成任务
- 🔔 系统托盘常驻，最小化到托盘
- 💾 本地 SQLite 存储，数据持久化
- 🚀 开机自启动（可选）

## 技术栈

| 层级 | 技术 |
|------|------|
| 框架 | Tauri 2 |
| 前端 | Vue 3 + Vite |
| 后端 | Rust |
| 数据库 | SQLite (rusqlite) |
| 通知 | tauri-plugin-notification |
| 自启动 | tauri-plugin-autostart |

## 开发环境要求

- [Node.js](https://nodejs.org/) >= 18
- [Rust](https://www.rust-lang.org/tools/install) >= 1.70
- [Tauri CLI](https://tauri.app/start/prerequisites/)

## 快速开始

```bash
# 克隆项目
git clone https://github.com/zhaojiejie513-ops/task-reminder.git
cd task-reminder

# 安装前端依赖
npm install

# 开发模式运行
npm run tauri dev

# 生产打包
npm run tauri build
```

打包产物位于 `src-tauri/target/release/bundle/nsis/`。

## 项目结构

```
task-reminder/
├── src/                    # 前端源码 (Vue 3)
│   ├── components/         # 组件
│   │   ├── TaskItem.vue        # 任务项
│   │   ├── CompletedItem.vue   # 已完成项
│   │   ├── PrioritySelector.vue# 优先级选择器
│   │   └── DateTimePicker.vue  # 日期时间选择器
│   ├── App.vue             # 主应用
│   ├── main.js             # 入口
│   └── styles.css          # 全局样式
├── src-tauri/              # 后端源码 (Rust)
│   ├── src/
│   │   ├── lib.rs          # 应用入口与插件注册
│   │   ├── commands.rs     # Tauri 命令
│   │   └── db.rs           # 数据库操作
│   ├── Cargo.toml
│   └── tauri.conf.json     # Tauri 配置
├── package.json
└── vite.config.js
```

## 使用说明

1. 在输入框输入任务内容，选择优先级和截止时间，点击 + 添加
2. 拖拽任务可调整顺序
3. 点击任务右侧圆圈完成任务
4. 点击铅笔图标可编辑任务内容、优先级和截止时间
5. 已完成任务可展开查看，支持恢复或永久删除
6. 关闭窗口后应用最小化到系统托盘

## License

MIT
