import type { ItemWithCategory } from '@/core/ipc.generated';

interface CategoryColumnProps {
  categoryName: string;
  items: ItemWithCategory[];
}

export function CategoryColumn({ categoryName, items }: CategoryColumnProps) {
  return (
    <div className="flex shrink-0 flex-col gap-3 rounded-lg bg-white/80 p-4 shadow-sm">
      <h2 className="text-xs font-semibold uppercase tracking-wider text-gray-500">
        {categoryName}
      </h2>
      <div className="flex flex-col gap-1.5">
        {items.map((item) => (
          <div key={item.id} className="flex items-baseline justify-between gap-4">
            <span className="text-sm text-gray-800">{item.label}</span>
            {item.value && (
              <span className="shrink-0 font-mono text-sm text-gray-400">
                {item.value}
              </span>
            )}
          </div>
        ))}
      </div>
    </div>
  );
}
