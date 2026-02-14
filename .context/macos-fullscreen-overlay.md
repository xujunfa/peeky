# macOS 浮层盖在全屏应用之上 — 方法论

> 基于对 KeyClu.app 二进制的逆向分析，提炼出可复刻的最小模式。

## 问题

macOS 全屏应用运行在独立的 Space 中，普通窗口无法显示在其上方。菜单栏常驻应用（如 Peeky）需要通过全局快捷键唤出浮层，要求浮层能盖在任何全屏应用之上。

## 核心原理

macOS 窗口系统通过三个维度决定窗口的可见性和层级：

1. **Window Level** — 窗口在 z-axis 上的层级
2. **Collection Behavior** — 窗口与 Spaces/全屏的关系
3. **Activation Policy** — 应用是否有资格"激活"并获取焦点

三者必须协同配置，缺一不可。

## 最小可复刻模式

### 1. Collection Behavior（关键配置）

```
collectionBehavior = [
    .moveToActiveSpace,     // 1<<1  窗口跟随用户切换到当前活跃 Space
    .stationary,            // 1<<4  窗口不随 Expose/Mission Control 移动
    .ignoresCycle,          // 1<<6  Cmd+` 切换窗口时跳过此窗口
    .fullScreenAuxiliary,   // 1<<8  允许作为全屏辅助窗口显示 ★核心★
    .auxiliary,             // 1<<17 标记为辅助窗口，不出现在 Window 菜单
]
```

**各 flag 的作用：**

| Flag | Bit | 作用 | 缺失后果 |
|------|-----|------|----------|
| `moveToActiveSpace` | 1<<1 | 窗口自动出现在用户当前所在的 Space | 切换 Space 后浮层消失 |
| `stationary` | 1<<4 | Mission Control 展开时窗口保持原位 | 浮层被 Mission Control 挤开 |
| `ignoresCycle` | 1<<6 | Cmd+\` 不会选中此窗口 | 用户无意中切到浮层 |
| `fullScreenAuxiliary` | 1<<8 | **允许出现在全屏 Space 中** | 全屏应用上方看不到浮层 |
| `auxiliary` | 1<<17 | 不出现在 Window 菜单和 App Switcher | 浮层干扰正常窗口管理 |

### 2. Window Level

```
window.level = .floating  // 值为 3
```

`floating`（3）足以盖在普通窗口和全屏应用之上。不需要更高的 level（如 `popUpMenu = 101`），过高的 level 会遮挡系统 UI（通知、Spotlight 等）。

**常用 level 参考：**

| Level | 值 | 用途 |
|-------|---|------|
| normal | 0 | 普通窗口 |
| floating | 3 | 浮动面板（推荐） |
| modalPanel | 8 | 模态对话框 |
| popUpMenu | 101 | 弹出菜单（过高，不推荐） |
| screenSaver | 1000 | 屏保 |

### 3. Activation Policy 切换（激活焦点的关键）

菜单栏应用通常使用 `Accessory` 策略（隐藏 Dock 图标）。但 Accessory 应用在全屏 Space 中无法主动获取焦点。

**解决方案 — 临时切换策略：**

```
// 显示浮层时
NSApp.setActivationPolicy(.regular)       // 1. 临时切到 Regular，获得激活资格
NSApp.activate(ignoringOtherApps: true)    // 2. 强制激活，抢夺焦点
window.orderFrontRegardless()              // 3. 强制窗口到最前
window.makeKeyAndOrderFront(nil)           // 4. 设为 key window 并显示
NSApp.setActivationPolicy(.accessory)      // 5. 立即切回 Accessory，隐藏 Dock 图标
```

这个 Regular -> activate -> Accessory 的切换必须在同一个 run loop tick 内完成，否则 Dock 图标会闪烁。

### 4. 隐藏浮层时的清理

```
window.hide()
NSApp.setActivationPolicy(.accessory)      // 确保 Accessory 状态
NSApp.deactivate()                         // 交还焦点给之前的全屏应用
```

`deactivate()` 很重要 — 不调用的话，全屏应用不会重新获得焦点。

## Tauri v2 中的实现

Tauri 通过 `with_webview` 暴露底层 `NSWindow` 指针，配合 `objc2` crate 调用 AppKit API。

### Cargo.toml 依赖

```toml
objc2 = "0.6"
objc2-app-kit = { version = "0.3", features = ["NSWindow", "NSApplication", "NSRunningApplication"] }
```

### Rust 实现

```rust
#[cfg(target_os = "macos")]
fn configure_overlay_for_fullscreen(window: &tauri::WebviewWindow) {
    let _ = window.with_webview(|webview| {
        use objc2::rc::Retained;
        use objc2::MainThreadMarker;
        use objc2_app_kit::{
            NSApplication, NSApplicationActivationPolicy, NSWindow,
            NSWindowCollectionBehavior,
        };

        unsafe {
            let ns_window_ptr = webview.ns_window();
            let ns_window: Retained<NSWindow> =
                Retained::retain(ns_window_ptr.cast()).unwrap();

            ns_window.setLevel(3); // floating

            let behavior = NSWindowCollectionBehavior::MoveToActiveSpace
                | NSWindowCollectionBehavior::Stationary
                | NSWindowCollectionBehavior::IgnoresCycle
                | NSWindowCollectionBehavior::FullScreenAuxiliary
                | NSWindowCollectionBehavior::Auxiliary;
            ns_window.setCollectionBehavior(behavior);

            ns_window.setCanHide(false);

            let mtm = MainThreadMarker::new().unwrap();
            let app = NSApplication::sharedApplication(mtm);
            app.setActivationPolicy(NSApplicationActivationPolicy::Regular);
            #[allow(deprecated)]
            app.activateIgnoringOtherApps(true);

            ns_window.orderFrontRegardless();
            ns_window.makeKeyAndOrderFront(None);

            app.setActivationPolicy(NSApplicationActivationPolicy::Accessory);
        }
    });
}
```

### 前置条件

- Tauri `features` 中需启用 `macos-private-api`（用于 `with_webview` 访问 NSWindow）
- 窗口配置需设置 `transparent: true`、`decorations: false`
- 应用启动时设置 `ActivationPolicy::Accessory`

## 注意事项

1. **`activateIgnoringOtherApps` 已在 macOS 14 标记 deprecated** — 目前仍可用，Apple 暂未提供等效的新 API。`NSApplication.activate()` 是新的替代，但在全屏场景下可能不够强势。需持续关注后续 macOS 版本的变化。

2. **`with_webview` 回调在主线程执行** — 可以安全地使用 `MainThreadMarker::new().unwrap()`。

3. **策略切换的时序** — `setActivationPolicy(.regular)` 和回切 `.accessory` 必须紧凑，中间不能有异步等待，否则 Dock 图标会短暂闪现。

4. **窗口 level 的选择** — `floating`（3）已足够。避免使用过高的 level，否则会遮挡系统级 UI（如通知中心、Spotlight）。

## 信息来源

- KeyClu.app 二进制逆向分析（反汇编确认 collectionBehavior 位掩码和窗口配置）
- Apple 文档：[NSWindowCollectionBehavior](https://developer.apple.com/documentation/appkit/nswindowcollectionbehavior)、[NSWindowLevel](https://developer.apple.com/documentation/appkit/nswindowlevel)
