# Active Context

## 当前状态（更新于 2026-02-13）

- Peeky MVP 全部 8 个任务已完成。
- 数据层：`categories` + `items` 表，Rust CRUD 命令 10 个，18 条 Rust 测试通过。
- Overlay 窗口：全屏毛玻璃浮层，多分栏布局，Esc 关闭 + 渐出动画。
- Main 窗口：左侧分栏管理 + 右侧条目 CRUD，基于 TanStack Query。
- 品牌化完成：Peeky / com.peeky.app / peeky.db。
- 全部验证命令通过（IPC check / TSC / Vitest 6 tests / Cargo 18 tests）。

## 下一步建议

- `pnpm tauri dev` 手动验收快捷键唤出/关闭、Tray 行为、CRUD 操作。
- 考虑后续功能：长按 Command 触发、拖拽排序、设置页面。

## 何时更新本文件

- 每完成一个阶段（Phase）后更新一次。
- 发生"可继续工作的上下文变化"（例如阻塞点、切换优先级、待办变化）时更新。
- 每次更新保持 5-15 行，避免写流水账。
