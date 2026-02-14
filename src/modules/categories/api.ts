import { typedInvoke } from '@/lib/tauri';
import type { CreateCategoryInput, UpdateCategoryInput } from '@/core/ipc.generated';

export function getCategories() {
  return typedInvoke('get_categories', {});
}

export function createCategory(input: CreateCategoryInput) {
  return typedInvoke('create_category', { input });
}

export function updateCategory(input: UpdateCategoryInput) {
  return typedInvoke('update_category', { input });
}

export function deleteCategory(id: number) {
  return typedInvoke('delete_category', { id });
}

export function reorderCategories(ids: number[]) {
  return typedInvoke('reorder_categories', { ids });
}
