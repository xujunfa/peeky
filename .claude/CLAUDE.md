# tauri-mac-starter

Tauri v2 + React + TypeScript 的 macOS 桌面模板仓库（业务中立）。

## 项目定位

- 提供可直接扩展的模板骨架，不包含 area/project/task/pomodoro 业务域。
- 当前 UI：
  - Main Window：模板首页（`src/windows/main/App.tsx`）
  - Timer Window：占位浮窗（`src/windows/timer/App.tsx`）

## 当前系统真相

- Dev 端口：`1430`（HMR `1431`）
- Tauri `devUrl`：`http://localhost:1430`
- 快捷键：
  - `Cmd+Shift+O`：切换 timer
  - `Cmd+Shift+L`：切换 main
- SQLite：`starter.db`
- Identifier：`com.tauri-mac-starter.app`
- IPC 契约真相：`src/core/ipc.generated.ts`

## Rust 命令契约（仅 5 个）

- `ping`
- `get_app_info`
- `get_settings`
- `set_settings`
- `update_tray_title`

> 新增命令必须先改 Rust + `generate_handler![]`，再跑 `pnpm gen:ipc`。

## 关键目录

- `src-tauri/src/lib.rs`：窗口/Tray/快捷键/命令注册
- `src-tauri/src/commands/app.rs`：app 基础命令
- `src-tauri/src/commands/settings.rs`：settings 读写命令
- `src-tauri/src/db.rs`：migration 注册 + db pool
- `src-tauri/migrations/001_init.sql`：最小 schema（`app_settings`）
- `src/core/ipc.ts`：`typedInvoke` 封装
- `src/core/ipc.generated.ts`：生成 IPC 类型（禁止手改）
- `src/modules/app/*`、`src/modules/settings/*`：前端 API 模块

## 常用命令

```bash
pnpm gen:ipc
pnpm gen:ipc:check
pnpm -s tsc -b --pretty false
pnpm test
cargo test --manifest-path src-tauri/Cargo.toml
pnpm dev
pnpm tauri dev
```

## Claude Code 工作流程

1. 先读 `src/core/ipc.generated.ts`，确认契约。
2. 命令变更顺序：Rust 实现 -> `generate_handler![]` -> `pnpm gen:ipc`。
3. 每次改动后最小验收：
   - `pnpm gen:ipc:check`
   - `pnpm -s tsc -b --pretty false`
   - `pnpm test`
   - `cargo test --manifest-path src-tauri/Cargo.toml`
4. 窗口行为改动时，手动验证快捷键与 Tray。

## Skills 使用指南（简版）

优先在“会影响架构或质量”的任务上主动使用 skills。

### 技术栈相关

- `tauri-v2`：Rust commands、Tauri 配置、权限/窗口行为。
- `tailwind-v4-shadcn`：Tailwind v4 + shadcn/ui 结构与样式规范。
- `jotai-expert`：设计 atom 结构与状态边界。
- `tanstack-query-best-practices`：服务端状态缓存、query/mutation 规范。
- `vercel-react-best-practices`：React 组件性能与渲染策略。

### 工程流程相关

- `brainstorming`：实现前澄清需求与边界。
- `writing-plans`：多步骤任务先拆解计划。
- `executing-plans`：按计划分步执行并校验。
- `test-driven-development`：功能/修复优先 test-first。
- `systematic-debugging`：故障定位先复现、再收敛根因。
- `github`：PR/Issue/CI 操作。
- `conventional-commits`：规范化提交信息。

## 低 token 文档策略

- 默认只读本文件（`./.claude/CLAUDE.md`）。
- 信息不足时按需读 1 个 `.context` 文件：
  - 进度：`.context/active_context.md`
  - 架构：`.context/architecture.md`
  - 决策：`.context/decisions.md`
  - 目标：`.context/design.md`
  - 计划：`.context/implementation_plan.md`
- 避免一次性读取全部 `.context/*.md`。

## 何时更新文档

- 运行事实变更（端口/快捷键/DB/命令）：更新本文件。
- 阶段进度变化：更新 `.context/active_context.md`。
- 模块边界变化：更新 `.context/architecture.md`。
- 技术取舍变化：更新 `.context/decisions.md`。
- 阶段目标或验收标准变化：更新 `.context/design.md` / `.context/implementation_plan.md`。
