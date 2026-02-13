# tauri-mac-starter

[English](./README.md) | [简体中文](./README.zh-CN.md)

一个面向 macOS 的生产可用桌面应用模板，基于 Tauri v2 + React + TypeScript。

## 模板能力

- Tauri v2 Rust 后端 + React 前端
- 多窗口结构（`main` + `timer`）
- Tray 支持 + 全局快捷键
- SQLite 集成（`starter.db`）
- 类型安全 IPC 工作流（`Rust command -> 生成 TS 类型 -> typedInvoke`）
- 最小模板命令集（`ping`、`get_app_info`、`get_settings`、`set_settings`、`update_tray_title`）

## 技术栈

- Tauri v2
- Rust + sqlx + tauri-plugin-sql
- React 19 + TypeScript + Vite 6
- Tailwind CSS v4 + shadcn/ui
- Jotai
- Vitest

## 快速开始

### 1. 安装依赖

```bash
pnpm install
```

### 2. 启动前端开发服务

```bash
pnpm dev
```

默认地址：`http://localhost:1430`

### 3. 启动桌面应用

```bash
pnpm tauri dev
```

## 如何基于模板创建新 Mac App

### 1. 用模板创建新仓库

- GitHub 上点击 `Use this template`
- 或本地复制后重新 `git init`

### 2. 先改应用标识（建议第一步完成）

- `package.json`
  - `name`、`description`
- `src-tauri/Cargo.toml`
  - `[package].name`、`description`、`[lib].name`
- `src-tauri/tauri.conf.json`
  - `productName`、`identifier`、窗口标题
- `src-tauri/icons/*`
  - 替换应用图标

## 目录结构

```text
src/
  windows/main/            主窗口 UI
  windows/timer/           浮窗/Timer UI
  modules/app/             app 命令前端封装
  modules/settings/        settings 命令前端封装
  core/ipc.ts              typedInvoke 封装
  core/ipc.generated.ts    自动生成 IPC 契约（禁止手改）

src-tauri/
  src/lib.rs               Tauri 初始化 + 快捷键 + 命令注册
  src/commands/app.rs      app 命令
  src/commands/settings.rs settings 命令
  src/db.rs                migration 注册 + DB pool
  migrations/001_init.sql  最小数据库 schema
```

## 新增 Command（推荐流程）

1. 在 Rust 侧新增命令（`src-tauri/src/commands/*.rs`）
2. 在 `src-tauri/src/lib.rs` 的 `generate_handler![]` 注册
3. 生成 IPC 类型：

```bash
pnpm gen:ipc
pnpm gen:ipc:check
```

4. 前端通过 `typedInvoke` 或 `src/modules/*` 封装调用

## 提交前质量门禁

```bash
pnpm gen:ipc:check
pnpm -s tsc -b --pretty false
pnpm test
pnpm fmt:rust:check
pnpm lint:rust
pnpm test:rust
```

## 当前默认运行配置

- Dev 端口：`1430`
- HMR 端口：`1431`
- Tauri dev URL：`http://localhost:1430`
- 全局快捷键：
  - `Cmd+Shift+O`：切换 timer 窗口
  - `Cmd+Shift+L`：切换 main 窗口
- 数据库文件：`starter.db`
- 应用 identifier：`com.tauri-mac-starter.app`

## 常见问题

### 端口冲突（并行运行多个 Tauri 项目时）

```bash
lsof -i :1430
lsof -i :1431
```

### 快捷键冲突

如果快捷键被其他应用占用：

- 先停止冲突应用，或
- 修改 `src-tauri/src/lib.rs` 中快捷键并同步文档

### IPC 校验失败

```bash
pnpm gen:ipc
pnpm gen:ipc:check
```

然后核对 `generate_handler![]` 是否与 `#[tauri::command]` 实现一致。
