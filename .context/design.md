# tauri-mac-starter 设计摘要

## 项目定位

`tauri-mac-starter` 是开源桌面模板仓库，目标是提供可直接扩展的生产级骨架，而非业务 Demo。

## 首版固定能力

- Tauri v2 + Rust
- React + TypeScript
- shadcn/ui + Tailwind CSS v4
- Jotai + TanStack Query
- SQLite（本地存储）
- 多窗口 + Tray + Global Shortcut
- typed IPC（Rust command -> TS types）

## 分阶段状态

- Phase 1：已完成（模板命令链与模块骨架）。
- Phase 2：已完成（UI 入口中性化，窗口/Tray/快捷键机制保留）。
- Phase 3：已完成（业务域与数据库耦合移除）。
- Phase 4：待执行（README、CI、发布准备、模板验证）。

## 何时更新本文件

- 产品目标、阶段定义、验收标准发生变化时。
- 新增或移除“首版固定能力”时。
