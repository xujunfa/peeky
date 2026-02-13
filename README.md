# tauri-mac-starter

[English](./README.md) | [简体中文](./README.zh-CN.md)

A production-ready macOS desktop app starter built with Tauri v2 + React + TypeScript.

## What You Get

- Tauri v2 Rust backend + React frontend
- Multi-window app structure (`main` + `timer`)
- Tray support + global shortcuts
- SQLite integration (`starter.db`)
- Type-safe IPC workflow (`Rust command -> generated TS types -> typedInvoke`)
- Minimal template commands (`ping`, `get_app_info`, `get_settings`, `set_settings`, `update_tray_title`)

## Tech Stack

- Tauri v2
- Rust + sqlx + tauri-plugin-sql
- React 19 + TypeScript + Vite 6
- Tailwind CSS v4 + shadcn/ui
- Jotai
- Vitest

## Quick Start

### 1. Install dependencies

```bash
pnpm install
```

### 2. Run frontend dev server

```bash
pnpm dev
```

Default local URL: `http://localhost:1430`

### 3. Run desktop app

```bash
pnpm tauri dev
```

## Use This Starter for a New Mac App

### 1. Create your new repo from this template

- GitHub: click `Use this template`
- Or clone/copy locally and initialize a new git repo

### 2. Update app identity first

Update these files before feature work:

- `package.json`
  - `name`, `description`
- `src-tauri/Cargo.toml`
  - `[package].name`, `description`, `[lib].name`
- `src-tauri/tauri.conf.json`
  - `productName`, `identifier`, window titles
- `src-tauri/icons/*`
  - replace icons if needed

## Project Layout

```text
src/
  windows/main/           Main window UI
  windows/timer/          Timer/floating window UI
  modules/app/            Frontend API wrappers for app commands
  modules/settings/       Frontend API wrappers for settings commands
  core/ipc.ts             typedInvoke wrapper
  core/ipc.generated.ts   generated command args/returns (do not edit)

src-tauri/
  src/lib.rs              Tauri setup + shortcuts + command registration
  src/commands/app.rs     app commands
  src/commands/settings.rs settings commands
  src/db.rs               migrations + DB pool
  migrations/001_init.sql minimal DB schema
```

## Add a New Command (Recommended Workflow)

1. Add command in Rust (`src-tauri/src/commands/*.rs`)
2. Register it in `src-tauri/src/lib.rs` (`generate_handler![]`)
3. Regenerate IPC types:

```bash
pnpm gen:ipc
pnpm gen:ipc:check
```

4. Call from frontend via `typedInvoke` or module APIs in `src/modules/*`

## Quality Gate (Run Before Commit)

```bash
pnpm gen:ipc:check
pnpm -s tsc -b --pretty false
pnpm test
pnpm fmt:rust:check
pnpm lint:rust
pnpm test:rust
```

## Current Runtime Defaults

- Dev port: `1430`
- HMR port: `1431`
- Tauri dev URL: `http://localhost:1430`
- Shortcuts:
  - `Cmd+Shift+O`: toggle timer window
  - `Cmd+Shift+L`: toggle main window
- Database file: `starter.db`
- Identifier: `com.tauri-mac-starter.app`

## Troubleshooting

### Port conflict (when running multiple Tauri projects)

```bash
lsof -i :1430
lsof -i :1431
```

### Global shortcut conflict

If another app already occupies shortcuts, either:

- stop the conflicting app, or
- change shortcuts in `src-tauri/src/lib.rs` and update docs

### IPC check fails

```bash
pnpm gen:ipc
pnpm gen:ipc:check
```

Then verify `generate_handler![]` matches actual `#[tauri::command]` functions.
