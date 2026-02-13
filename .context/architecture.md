# Architecture

## 当前架构（模板态）

- Rust 平台层：`src-tauri/src/lib.rs`
  - 窗口管理、Tray、全局快捷键、命令注册。
- 命令层：`src-tauri/src/commands/app.rs` + `src-tauri/src/commands/settings.rs`
  - 仅保留 5 个模板命令。
- 数据层：`src-tauri/src/db.rs` + `src-tauri/migrations/001_init.sql`
  - SQLite `starter.db`，仅 `app_settings` 表。
- 前端 IPC 层：`src/core/ipc.ts` + `src/core/ipc.generated.ts`
  - `typedInvoke` + 自动生成契约。
- 前端模块层：`src/modules/app/*` + `src/modules/settings/*`
  - 封装模板命令调用。
- UI 层：`src/windows/main/App.tsx` + `src/windows/timer/App.tsx`
  - 中性首页 + timer 占位页。

## 何时更新本文件

- 模块边界变化（新增/删除核心目录或职责迁移）。
- 命令层、数据层、IPC 层关系发生变化。
- 窗口机制、Tray 或快捷键机制出现结构性变化。
