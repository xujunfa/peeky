import { describe, it, expect } from 'vitest';
import { COMMAND_NAMES } from './ipc.generated';

describe('ipc generated types', () => {
  it('should only include template command names', () => {
    expect(COMMAND_NAMES).toEqual([
      'ping',
      'get_app_info',
      'get_settings',
      'set_settings',
      'update_tray_title',
    ]);
  });
});
