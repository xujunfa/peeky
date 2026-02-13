# Active Context

## 当前状态（更新于 2026-02-13）

- Phase 1 已完成：模板命令链 `ping/get_app_info/get_settings/set_settings` 已可用。
- Phase 2 已完成：主窗口中性化、timer 占位化、快捷键改为窗口切换语义。
- Phase 3 已完成：业务域命令与业务表已移除，DB 切换为 `starter.db`。
- 当前分支代码可通过最小验收命令组（IPC/TS/Test/Cargo test）。

## 下一步建议

- 进入 Phase 4：README、LICENSE、CI、模板发布校验。

## 何时更新本文件

- 每完成一个阶段（Phase）后更新一次。
- 发生“可继续工作的上下文变化”（例如阻塞点、切换优先级、待办变化）时更新。
- 每次更新保持 5-15 行，避免写流水账。
