import { typedInvoke } from '@/lib/tauri';

export function ping() {
  return typedInvoke('ping', {});
}

export function getAppInfo() {
  return typedInvoke('get_app_info', {});
}
