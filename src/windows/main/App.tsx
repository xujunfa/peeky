import { useEffect, useState } from 'react';
import { getAppInfo } from '@/modules/app';
import { getSettings } from '@/modules/settings';

type AppInfo = Awaited<ReturnType<typeof getAppInfo>>;
type Settings = Awaited<ReturnType<typeof getSettings>>;

function App() {
  const [appInfo, setAppInfo] = useState<AppInfo | null>(null);
  const [settings, setSettings] = useState<Settings | null>(null);

  useEffect(() => {
    getAppInfo().then(setAppInfo).catch(() => null);
    getSettings().then(setSettings).catch(() => null);
  }, []);

  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-50 via-stone-50 to-emerald-50 p-8 text-slate-900">
      <main className="mx-auto flex h-full max-w-4xl flex-col gap-6 rounded-3xl border border-slate-200 bg-white/90 p-8 shadow-sm">
        <header className="space-y-2">
          <p className="text-xs font-semibold uppercase tracking-[0.2em] text-emerald-700">
            tauri-mac-starter
          </p>
          <h1 className="text-3xl font-semibold tracking-tight">
            Desktop Template Home
          </h1>
          <p className="text-sm text-slate-600">
            A neutral starter for multi-window macOS apps built with Tauri v2
            and React.
          </p>
        </header>

        <section className="grid gap-4 md:grid-cols-2">
          <article className="rounded-2xl border border-slate-200 bg-slate-50 p-4">
            <h2 className="text-sm font-semibold text-slate-800">
              Core capabilities
            </h2>
            <ul className="mt-3 space-y-1 text-sm text-slate-600">
              <li>Tauri v2 + Rust command IPC</li>
              <li>React + TypeScript + Vite</li>
              <li>Jotai + TanStack Query</li>
              <li>SQLite + typed IPC generation</li>
              <li>Multi-window + Tray + Global Shortcut</li>
            </ul>
          </article>

          <article className="rounded-2xl border border-slate-200 bg-white p-4">
            <h2 className="text-sm font-semibold text-slate-800">
              Runtime snapshot
            </h2>
            <dl className="mt-3 space-y-1 text-sm text-slate-600">
              <div>
                <dt className="inline font-medium text-slate-800">App:</dt>{' '}
                <dd className="inline">{appInfo?.name ?? '-'}</dd>
              </div>
              <div>
                <dt className="inline font-medium text-slate-800">Version:</dt>{' '}
                <dd className="inline">{appInfo?.version ?? '-'}</dd>
              </div>
              <div>
                <dt className="inline font-medium text-slate-800">Locale:</dt>{' '}
                <dd className="inline">{settings?.locale ?? '-'}</dd>
              </div>
              <div>
                <dt className="inline font-medium text-slate-800">Theme:</dt>{' '}
                <dd className="inline">{settings?.theme ?? '-'}</dd>
              </div>
            </dl>
          </article>
        </section>

        <section className="rounded-2xl border border-dashed border-slate-300 bg-emerald-50/50 p-4 text-sm text-slate-700">
          Next step: add your own domain commands in Rust, run{' '}
          <code>pnpm gen:ipc</code>, and consume them from{' '}
          <code>typedInvoke</code>-based modules.
        </section>
      </main>
    </div>
  );
}

export default App;
