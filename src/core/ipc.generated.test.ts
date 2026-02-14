import { describe, it, expect } from 'vitest';
import { COMMAND_NAMES } from './ipc.generated';

describe('ipc generated types', () => {
  it('should include all registered command names', () => {
    expect(COMMAND_NAMES).toEqual([
      'ping',
      'get_app_info',
      'get_settings',
      'set_settings',
      'get_categories',
      'create_category',
      'update_category',
      'delete_category',
      'reorder_categories',
      'get_items',
      'get_all_items',
      'create_item',
      'update_item',
      'delete_item',
      'update_tray_title',
    ]);
  });
});
