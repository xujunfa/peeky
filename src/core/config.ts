export const TEMPLATE_INFO = {
  name: 'tauri-mac-starter',
  version: '0.1.0',
  description: 'A production-ready Tauri v2 + React template for desktop apps',
} as const;

export const DEFAULT_WINDOW_LABELS = {
  main: 'main',
  timer: 'timer',
} as const;

export const DEFAULT_SHORTCUTS = {
  toggleTimer: 'Cmd+Shift+O',
  toggleMain: 'Cmd+Shift+L',
} as const;
