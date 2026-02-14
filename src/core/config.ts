export const TEMPLATE_INFO = {
  name: 'Peeky',
  version: '0.1.0',
  description: 'macOS menu bar memo overlay app',
} as const;

export const DEFAULT_WINDOW_LABELS = {
  main: 'main',
  overlay: 'overlay',
} as const;

export const DEFAULT_SHORTCUTS = {
  toggleOverlay: 'Cmd+Shift+O',
  toggleMain: 'Cmd+Shift+L',
} as const;
