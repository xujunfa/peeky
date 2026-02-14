# Peeky Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 将 tauri-mac-starter 模板转变为 Peeky——macOS 菜单栏常驻备忘浮层应用。

**Architecture:** Overlay 窗口（全屏、透明、无边框）通过全局快捷键唤出，展示多分栏备忘信息；Main 窗口用于管理分栏和条目。数据层使用 SQLite（categories + items 表），Rust 命令提供 CRUD，前端通过 typedInvoke 调用。

**Tech Stack:** Tauri v2 / Rust / React 19 / TypeScript / Tailwind v4 + shadcn/ui / Jotai / TanStack Query / SQLite

---

## Phase 1: 数据层（Rust + DB）

### Task 1: 新增 categories 和 items 迁移

**Files:**
- Create: `src-tauri/migrations/002_peeky_domain.sql`
- Modify: `src-tauri/src/db.rs`

**Step 1: 编写迁移 SQL**

```sql
-- 002_peeky_domain.sql
CREATE TABLE IF NOT EXISTS categories (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  sort_order INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL DEFAULT (datetime('now')),
  updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS items (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  category_id INTEGER NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
  label TEXT NOT NULL,
  value TEXT NOT NULL DEFAULT '',
  sort_order INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL DEFAULT (datetime('now')),
  updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
```

**Step 2: 在 `db.rs` 注册迁移**

在 `migrations()` Vec 中追加 version 2。

**Step 3: 验证**

```bash
cargo test --manifest-path src-tauri/Cargo.toml
```

**Step 4: Commit**

```bash
git commit -m "feat(db): add categories and items tables"
```

---

### Task 2: 新增 categories CRUD 命令

**Files:**
- Create: `src-tauri/src/commands/categories.rs`
- Modify: `src-tauri/src/commands/mod.rs`
- Modify: `src-tauri/src/lib.rs`（`generate_handler![]`）

**实现命令：**
- `get_categories` → `Vec<Category>`
- `create_category(name)` → `Category`
- `update_category(id, name?, sort_order?)` → `Category`
- `delete_category(id)` → `()`
- `reorder_categories(ids: Vec<i64>)` → `()`

参考 `settings.rs` 的 `_by_pool` 模式，编写可测试的纯函数 + `#[tauri::command]` 薄封装。为每个函数写 `#[tokio::test]`（内存 SQLite）。

**验证：**
```bash
cargo test --manifest-path src-tauri/Cargo.toml
```

**Commit：** `feat(commands): add categories CRUD`

---

### Task 3: 新增 items CRUD 命令

**Files:**
- Create: `src-tauri/src/commands/items.rs`
- Modify: `src-tauri/src/commands/mod.rs`
- Modify: `src-tauri/src/lib.rs`（`generate_handler![]`）

**实现命令：**
- `get_items(category_id)` → `Vec<Item>`
- `get_all_items` → `Vec<ItemWithCategory>`（浮层渲染用，JOIN 返回）
- `create_item(category_id, label, value?)` → `Item`
- `update_item(id, label?, value?, sort_order?)` → `Item`
- `delete_item(id)` → `()`

同样 `_by_pool` + tokio::test。

**验证：**
```bash
cargo test --manifest-path src-tauri/Cargo.toml
```

**Commit：** `feat(commands): add items CRUD`

---

### Task 4: 生成 IPC 类型 + 前端模块

**Step 1:** 运行 `pnpm gen:ipc`，确认 `ipc.generated.ts` 更新。

**Step 2:** 创建前端 API 模块：
- Create: `src/modules/categories/api.ts`（封装 typedInvoke 调用）
- Create: `src/modules/categories/index.ts`
- Create: `src/modules/items/api.ts`
- Create: `src/modules/items/index.ts`

**Step 3:** 全量验证：
```bash
pnpm gen:ipc:check
pnpm -s tsc -b --pretty false
pnpm test
cargo test --manifest-path src-tauri/Cargo.toml
```

**Commit：** `feat(ipc): generate peeky domain types and frontend API modules`

---

## Phase 2: Overlay 窗口

### Task 5: 配置 Overlay 窗口

**Files:**
- Modify: `src-tauri/tauri.conf.json`
- Modify: `src-tauri/src/lib.rs`
- Modify: `src-tauri/capabilities/default.json`
- Create: `overlay.html`（新入口）
- Create: `src/windows/overlay/main.tsx`
- Create: `src/windows/overlay/App.tsx`
- Modify: `vite.config.ts`（添加 overlay 入口）

**重点：**
- 将模板 `timer` 窗口替换为 `overlay` 窗口（全屏、transparent、decorations=false、alwaysOnTop）。
- 快捷键从 `Cmd+Shift+O` 改为切换 overlay 窗口。
- overlay.html / main.tsx 最小骨架，App.tsx 先渲染占位文本。

**验证：**
```bash
pnpm -s tsc -b --pretty false
pnpm test
```

**Commit：** `feat(overlay): add overlay window skeleton and config`

---

### Task 6: Overlay UI——毛玻璃 + 多分栏布局

**Files:**
- Modify: `src/windows/overlay/App.tsx`
- Create: `src/windows/overlay/components/CategoryColumn.tsx`

**实现：**
- 使用 `get_all_items` 获取数据（TanStack Query）。
- 全屏毛玻璃背景（`backdrop-blur-xl bg-black/30`）。
- 多分栏布局（CSS Grid / flex），每个分栏展示 category name + items 列表。
- Esc 键关闭（`window.close()` 或 invoke hide）。
- 渐入/渐出动画（Tailwind `animate-` + `transition`，约 200ms）。

**验证：** `pnpm tauri dev` 手动测试快捷键唤出/关闭。

**Commit：** `feat(overlay): frosted glass multi-column layout with animation`

---

## Phase 3: Main 窗口管理界面

### Task 7: Main 窗口——分栏管理 UI

**Files:**
- Modify: `src/windows/main/App.tsx`
- Create: `src/windows/main/components/CategoryList.tsx`
- Create: `src/windows/main/components/CategoryForm.tsx`
- Create: `src/windows/main/components/ItemList.tsx`
- Create: `src/windows/main/components/ItemForm.tsx`

**实现：**
- 左侧 CategoryList：展示所有分栏，支持增删改、拖拽排序。
- 右侧 ItemList：选中分栏后展示条目，支持增删改。
- 使用 TanStack Query 做数据获取 + mutation。
- shadcn/ui Dialog 做新增/编辑表单。

**验证：** `pnpm tauri dev` 手动测试 CRUD 操作。

**Commit：** `feat(main): category and item management UI`

---

## Phase 4: 品牌化收尾

### Task 8: 模板残留清理 + 品牌化

**Files:**
- Modify: `src-tauri/tauri.conf.json`（productName → Peeky, identifier → com.peeky.app, db → peeky.db）
- Modify: `src-tauri/src/lib.rs`（tray tooltip/title、db 文件名）
- Modify: `src-tauri/src/commands/app.rs`（AppInfo name/identifier）
- Modify: `.claude/CLAUDE.md`（更新运行事实）
- 删除不再需要的模板文件/占位组件。

**验证：**
```bash
pnpm gen:ipc:check
pnpm -s tsc -b --pretty false
pnpm test
cargo test --manifest-path src-tauri/Cargo.toml
```

**Commit：** `chore: rebrand to Peeky and clean up template remnants`

---

## 验收标准

- [ ] `Cmd+Shift+O` 唤出全屏毛玻璃浮层，展示分栏和条目
- [ ] `Esc` / 再次快捷键关闭浮层，带渐出动画
- [ ] Main 窗口可增删改分栏与条目
- [ ] Tray 点击打开 Main 窗口
- [ ] 全部验证命令通过：`pnpm gen:ipc:check && pnpm -s tsc -b --pretty false && pnpm test && cargo test --manifest-path src-tauri/Cargo.toml`

## 何时更新本文件

- 开始新阶段实施前。
- 阶段内任务拆分发生明显变化时。
- 验收标准或执行顺序调整时。
