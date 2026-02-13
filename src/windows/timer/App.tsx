function App() {
  return (
    <div className="h-screen bg-transparent p-2">
      <div
        className="flex h-full flex-col justify-between rounded-2xl border border-white/20 bg-gradient-to-br from-slate-900/90 to-emerald-900/80 p-5 text-white shadow-lg backdrop-blur"
        data-tauri-drag-region
      >
        <div className="space-y-2" data-tauri-drag-region>
          <p className="text-xs uppercase tracking-[0.2em] text-emerald-200">
            tauri-mac-starter
          </p>
          <h1 className="text-lg font-semibold">Timer Placeholder</h1>
          <p className="text-sm text-emerald-100/80">
            This floating window is reserved for your custom productivity flow.
          </p>
        </div>

        <div className="rounded-xl border border-white/20 bg-black/20 p-3 text-xs text-emerald-100/90">
          Global shortcut support is active:
          <br />
          Cmd+Shift+O toggles this window.
          <br />
          Cmd+Shift+L toggles the main window.
        </div>
      </div>
    </div>
  );
}

export default App;
