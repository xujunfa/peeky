import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { getItems, createItem, deleteItem, updateItem } from '@/modules/items';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { useState } from 'react';
import type { Category } from '@/core/ipc.generated';

interface ItemListProps {
  category: Category;
}

export function ItemList({ category }: ItemListProps) {
  const queryClient = useQueryClient();
  const [newLabel, setNewLabel] = useState('');
  const [newValue, setNewValue] = useState('');
  const [editingId, setEditingId] = useState<number | null>(null);
  const [editLabel, setEditLabel] = useState('');
  const [editValue, setEditValue] = useState('');

  const { data: items = [] } = useQuery({
    queryKey: ['items', category.id],
    queryFn: () => getItems(category.id),
  });

  const invalidate = () => {
    queryClient.invalidateQueries({ queryKey: ['items', category.id] });
    queryClient.invalidateQueries({ queryKey: ['all-items'] });
  };

  const createMutation = useMutation({
    mutationFn: () =>
      createItem({
        category_id: category.id,
        label: newLabel.trim(),
        value: newValue.trim() || null,
      }),
    onSuccess: () => {
      invalidate();
      setNewLabel('');
      setNewValue('');
    },
  });

  const deleteMutation = useMutation({
    mutationFn: deleteItem,
    onSuccess: invalidate,
  });

  const updateMutation = useMutation({
    mutationFn: (args: { id: number; label: string; value: string }) =>
      updateItem({ id: args.id, label: args.label, value: args.value, sort_order: null }),
    onSuccess: () => {
      invalidate();
      setEditingId(null);
    },
  });

  const handleCreate = () => {
    if (!newLabel.trim()) return;
    createMutation.mutate();
  };

  const startEdit = (id: number, label: string, value: string) => {
    setEditingId(id);
    setEditLabel(label);
    setEditValue(value);
  };

  const saveEdit = () => {
    if (editingId === null || !editLabel.trim()) return;
    updateMutation.mutate({ id: editingId, label: editLabel.trim(), value: editValue.trim() });
  };

  return (
    <div className="flex h-full flex-col gap-3">
      <h2 className="text-sm font-semibold uppercase tracking-wider text-slate-500">
        {category.name} — Items
      </h2>

      <div className="flex gap-2">
        <Input
          value={newLabel}
          onChange={(e) => setNewLabel(e.target.value)}
          placeholder="Label..."
          className="h-8 text-sm"
        />
        <Input
          value={newValue}
          onChange={(e) => setNewValue(e.target.value)}
          onKeyDown={(e) => e.key === 'Enter' && handleCreate()}
          placeholder="Value (optional)..."
          className="h-8 text-sm"
        />
        <Button size="sm" onClick={handleCreate} disabled={!newLabel.trim()}>
          Add
        </Button>
      </div>

      <div className="flex flex-1 flex-col gap-1 overflow-auto">
        {items.map((item) => (
          <div
            key={item.id}
            className="group flex items-center justify-between rounded-lg px-3 py-2 text-sm hover:bg-slate-50"
          >
            {editingId === item.id ? (
              <div className="flex flex-1 gap-2">
                <Input
                  value={editLabel}
                  onChange={(e) => setEditLabel(e.target.value)}
                  className="h-7 text-sm"
                  autoFocus
                />
                <Input
                  value={editValue}
                  onChange={(e) => setEditValue(e.target.value)}
                  onKeyDown={(e) => e.key === 'Enter' && saveEdit()}
                  className="h-7 text-sm"
                />
                <Button size="sm" variant="outline" onClick={saveEdit}>
                  Save
                </Button>
                <Button
                  size="sm"
                  variant="ghost"
                  onClick={() => setEditingId(null)}
                >
                  Cancel
                </Button>
              </div>
            ) : (
              <>
                <div className="flex items-baseline gap-3">
                  <span className="text-slate-800">{item.label}</span>
                  {item.value && (
                    <span className="font-mono text-xs text-slate-400">
                      {item.value}
                    </span>
                  )}
                </div>
                <div className="flex gap-2 opacity-0 transition-opacity group-hover:opacity-100">
                  <button
                    className="text-xs text-slate-400 hover:text-slate-700"
                    onClick={() => startEdit(item.id, item.label, item.value)}
                  >
                    Edit
                  </button>
                  <button
                    className="text-xs text-slate-400 hover:text-red-500"
                    onClick={() => deleteMutation.mutate(item.id)}
                  >
                    Delete
                  </button>
                </div>
              </>
            )}
          </div>
        ))}
        {items.length === 0 && (
          <p className="py-4 text-center text-sm text-slate-400">
            No items in this category
          </p>
        )}
      </div>
    </div>
  );
}
