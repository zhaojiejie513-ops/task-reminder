---
name: code-standards
description: 为项目设置完整的代码规范体系，包括 linter、formatter 配置文件和代码规范文档。当用户提到"代码规范"、"代码风格"、"设置 ESLint"、"配置 Prettier"、"rustfmt"、"格式化配置"、"lint 配置"、"编码标准"、"code style"、"coding standards"时触发此 skill。即使用户只是说"帮我整理一下代码风格"或"项目需要统一规范"，也应该使用这个 skill。
---

# Code Standards — 项目代码规范配置

为项目生成完整的代码规范体系：配置文件 + 规范文档，覆盖前端和后端。

## 工作流程

### 1. 分析项目技术栈

先确认项目使用的语言和框架，然后按需生成对应的配置。

### 2. 根据技术栈生成配置

#### 前端（JavaScript/TypeScript/CSS）

生成以下文件：

**`eslint.config.mjs`** — JavaScript/TypeScript 静态检查（ESLint v9+ flat config 格式）
```js
import js from '@eslint/js';

export default [
  js.configs.recommended,
  {
    languageOptions: {
      ecmaVersion: 'latest',
      sourceType: 'module',
      globals: {
        window: 'readonly',
        document: 'readonly',
        console: 'readonly',
        setTimeout: 'readonly',
        setInterval: 'readonly',
        clearInterval: 'readonly',
      },
    },
    rules: {
      'no-unused-vars': 'warn',
      'no-console': 'off',
      'eqeqeq': 'error',
      'no-var': 'error',
      'prefer-const': 'warn',
      'curly': ['error', 'all'],
      'no-throw-literal': 'error',
    },
  },
];
```

如果项目使用 TypeScript，安装 `typescript-eslint` 并在配置中引入其推荐规则。

**`.prettierrc`** — 代码格式化
```json
{
  "semi": true,
  "singleQuote": true,
  "tabWidth": 2,
  "trailingComma": "es5",
  "printWidth": 100,
  "bracketSpacing": true,
  "endOfLine": "lf"
}
```

**`.prettierignore`**
```
node_modules/
dist/
build/
target/
*.min.js
```

#### 后端（Rust）

**`rustfmt.toml`** — Rust 代码格式化（放在 src-tauri/ 或项目根目录）
```toml
edition = "2021"
max_width = 100
tab_spaces = 4
use_field_init_shorthand = true
use_try_shorthand = true
newline_style = "Unix"
```

**`clippy.toml`** — Clippy 静态分析配置
```toml
too-many-arguments-threshold = 6
type-complexity-threshold = 300
```

在 `src-tauri/Cargo.toml` 中添加 Clippy lint 级别（如果尚未存在）：
```toml
[lints.clippy]
pedantic = { level = "warn", priority = -1 }
unwrap_used = "warn"
expect_used = "warn"
```

#### 通用配置

**`.editorconfig`** — 编辑器统一配置（放在项目根目录）
```ini
root = true

[*]
charset = utf-8
end_of_line = lf
indent_style = space
indent_size = 2
insert_final_newline = true
trim_trailing_whitespace = true

[*.rs]
indent_size = 4

[*.md]
trim_trailing_whitespace = false
```

### 3. 添加 npm scripts

在 `package.json` 的 `scripts` 中添加：
```json
{
  "lint": "eslint src/",
  "lint:fix": "eslint src/ --fix",
  "format": "prettier --write src/",
  "format:check": "prettier --check src/"
}
```

### 4. 安装依赖

前端依赖（作为 devDependencies）：
```bash
npm install -D eslint @eslint/js prettier
```

如果有 TypeScript：
```bash
npm install -D @typescript-eslint/parser @typescript-eslint/eslint-plugin
```

### 5. 生成代码规范文档

在项目根目录创建 `CODE_STANDARDS.md`，内容包括：

- **命名规范**：变量、函数、文件的命名约定
- **代码风格**：缩进、换行、括号等格式要求
- **注释规范**：何时写注释、注释格式
- **Git 提交规范**：commit message 格式（推荐 Conventional Commits）
- **文件组织**：目录结构约定

文档应该简洁实用，不要写成论文。根据项目实际技术栈调整内容。

## 注意事项

- 检查项目是否已有部分配置，有的话在其基础上补充而非覆盖
- 配置应该务实，不要过于严格导致开发体验差
- Rust 项目的 clippy pedantic 设为 warn 而非 deny，给开发者留余地
- 如果项目使用了框架（React、Vue 等），ESLint 配置要加上对应插件
- 生成完配置后，跑一次 lint 确认没有大量误报，如果有则调整规则
