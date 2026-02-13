import { typedInvoke } from '@/lib/tauri';
import type { CommandArg } from '@/core/ipc';

export function getSettings() {
  return typedInvoke('get_settings', {});
}

export function setSettings(input: CommandArg<'set_settings'>['input']) {
  return typedInvoke('set_settings', { input });
}
