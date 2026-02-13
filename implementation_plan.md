# tauri-mac-starter Phase Implementation Plan

> **For Codex:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Deliver the template skeleton defined in `.context/design.md`, and start Phase 1 by establishing neutral app/settings modules and a minimal typed IPC command chain.

**Architecture:** Phase 1 builds a thin vertical slice from Rust commands to TypeScript module boundaries without deleting legacy business code yet. Rust commands become the single source of IPC contracts, while frontend modules consume a typed invoke surface. Work is sequenced in TDD order to keep behavior verifiable at each checkpoint.

**Tech Stack:** Tauri v2 (Rust), React + TypeScript, Jotai, TanStack Query, Tailwind v4, shadcn/ui, SQLite, typed IPC generation.

---

## Phase 1 Scope (from design)

- Add neutral module directories: `src/modules/app`, `src/modules/settings`.
- Add minimal common commands: `ping`, `get_app_info`, `get_settings`, `set_settings`.
- Wire commands into command registry (`generate_handler![]` target shape).
- Keep type-generation path ready (`gen:ipc`/`gen:ipc:check` target shape).

## Task 1: Create Rust command contract + tests

**Files:**
- Create: `src-tauri/Cargo.toml`
- Create: `src-tauri/src/lib.rs`
- Create: `src-tauri/src/commands/mod.rs`
- Create: `src-tauri/src/commands/app.rs`
- Create: `src-tauri/src/commands/settings.rs`

**Steps (TDD):**
1. Write failing tests for all four commands and their response shapes.
2. Run `cargo test` in `src-tauri` and verify red state.
3. Implement minimal structs/functions to satisfy tests.
4. Re-run `cargo test` and verify green state.

## Task 2: Add frontend module skeleton for Phase 1

**Files:**
- Create: `src/modules/app/index.ts`
- Create: `src/modules/settings/index.ts`
- Create: `src/lib/ipc/contracts.ts`

**Steps:**
1. Define typed command names and payload/result contracts.
2. Export module-level API wrappers for app/settings commands.
3. Keep implementation neutral and business-free.

## Task 3: Prepare typed IPC generation touchpoints

**Files:**
- Create: `scripts/gen-ipc.mjs`
- Create: `scripts/check-ipc.mjs`
- Create: `package.json`

**Steps:**
1. Add placeholder `gen:ipc` and `gen:ipc:check` scripts to codify expected workflow.
2. Make checks deterministic for CI integration in later phases.
3. Document current limitations in comments (until full Tauri app lands).

## Task 4: Verification and handoff

**Checks:**
1. `cargo test` under `src-tauri` passes.
2. `node scripts/check-ipc.mjs` passes.
3. Directory skeleton exists for `src/modules/app` and `src/modules/settings`.

## Execution order

1. Task 1 (start now)
2. Task 2
3. Task 3
4. Task 4
