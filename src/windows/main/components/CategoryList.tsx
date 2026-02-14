import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { getCategories, createCategory, deleteCategory } from '@/modules/categories';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { useState } from 'react';
import type { Category } from '@/core/ipc.generated';

interface CategoryListProps {
  selectedId: number | null;
  onSelect: (category: Category) => void;
}

export function CategoryList({ selectedId, onSelect }: CategoryListProps) {
  const queryClient = useQueryClient();
  const [newName, setNewName] = useState('');

  const { data: categories = [] } = useQuery({
    queryKey: ['categories'],
    queryFn: getCategories,
  });

  const createMutation = useMutation({
    mutationFn: (name: string) => createCategory({ name }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['categories'] });
      setNewName('');
    },
  });

  const deleteMutation = useMutation({
    mutationFn: deleteCategory,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['categories'] });
      queryClient.invalidateQueries({ queryKey: ['items'] });
    },
  });

  const handleCreate = () => {
    const trimmed = newName.trim();
    if (!trimmed) return;
    createMutation.mutate(trimmed);
  };

  return (
    <div className="flex h-full flex-col gap-3">
      <h2 className="text-sm font-semibold uppercase tracking-wider text-slate-500">
        Categories
      </h2>

      <div className="flex gap-2">
        <Input
          value={newName}
          onChange={(e) => setNewName(e.target.value)}
          onKeyDown={(e) => e.key === 'Enter' && handleCreate()}
          placeholder="New category..."
          className="h-8 text-sm"
        />
        <Button size="sm" onClick={handleCreate} disabled={!newName.trim()}>
          Add
        </Button>
      </div>

      <div className="flex flex-1 flex-col gap-1 overflow-auto">
        {categories.map((cat) => (
          <div
            key={cat.id}
            className={`group flex cursor-pointer items-center justify-between rounded-lg px-3 py-2 text-sm transition-colors ${
              selectedId === cat.id
                ? 'bg-slate-900 text-white'
                : 'text-slate-700 hover:bg-slate-100'
            }`}
            onClick={() => onSelect(cat)}
          >
            <span>{cat.name}</span>
            <button
              className={`text-xs opacity-0 transition-opacity group-hover:opacity-100 ${
                selectedId === cat.id
                  ? 'text-white/60 hover:text-white'
                  : 'text-slate-400 hover:text-red-500'
              }`}
              onClick={(e) => {
                e.stopPropagation();
                deleteMutation.mutate(cat.id);
              }}
            >
              Delete
            </button>
          </div>
        ))}
        {categories.length === 0 && (
          <p className="py-4 text-center text-sm text-slate-400">
            No categories yet
          </p>
        )}
      </div>
    </div>
  );
}
