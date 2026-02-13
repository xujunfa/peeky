# Implementation Plan (Current)

## 当前执行目标

推进 Phase 4（开源发布准备）：

1. 完整 README（快速开始、结构说明、IPC 扩展流程）。
2. 配置 CI（`gen:ipc:check` + `tsc` + `test`）。
3. 校验发布资料（LICENSE/.gitignore/必要文档）。
4. 模板仓库演练（Use this template 后可直接运行）。

## 验收命令

```bash
pnpm gen:ipc:check
pnpm -s tsc -b --pretty false
pnpm test
cargo test --manifest-path src-tauri/Cargo.toml
```

## 何时更新本文件

- 开始新阶段实施前。
- 阶段内任务拆分发生明显变化时。
- 验收标准或执行顺序调整时。
