# Architecture

## 当前架构（Peeky MVP）

- Rust 平台层：`src-tauri/src/lib.rs`
  - 窗口管理（overlay + main）、Tray、全局快捷键、命令注册。
- 命令层：
  - `src-tauri/src/commands/app.rs`：ping, get_app_info
  - `src-tauri/src/commands/settings.rs`：settings 读写
  - `src-tauri/src/commands/categories.rs`：categories CRUD（5 命令）
  - `src-tauri/src/commands/items.rs`：items CRUD（5 命令）
- 数据层：`src-tauri/src/db.rs` + migrations/
  - SQLite `peeky.db`，3 张表：`app_settings`、`categories`、`items`。
- 前端 IPC 层：`src/core/ipc.ts` + `src/core/ipc.generated.ts`
  - `typedInvoke` + 自动生成契约（15 个命令）。
- 前端模块层：`src/modules/{app,settings,categories,items}/`
  - 封装 typedInvoke 调用。
- UI 层：
  - `src/windows/overlay/App.tsx`：全屏毛玻璃浮层，多分栏备忘展示。
  - `src/windows/main/App.tsx`：左右分栏管理界面（CategoryList + ItemList）。

## 何时更新本文件

- 模块边界变化（新增/删除核心目录或职责迁移）。
- 命令层、数据层、IPC 层关系发生变化。
- 窗口机制、Tray 或快捷键机制出现结构性变化。
