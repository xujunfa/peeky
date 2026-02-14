import { typedInvoke } from '@/lib/tauri';
import type { CreateItemInput, UpdateItemInput } from '@/core/ipc.generated';

export function getItems(categoryId: number) {
  return typedInvoke('get_items', { categoryId });
}

export function getAllItems() {
  return typedInvoke('get_all_items', {});
}

export function createItem(input: CreateItemInput) {
  return typedInvoke('create_item', { input });
}

export function updateItem(input: UpdateItemInput) {
  return typedInvoke('update_item', { input });
}

export function deleteItem(id: number) {
  return typedInvoke('delete_item', { id });
}
