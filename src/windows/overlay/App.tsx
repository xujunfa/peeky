import { useEffect, useState, useMemo } from 'react';
import { useQuery } from '@tanstack/react-query';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { getAllItems } from '@/modules/items';
import { CategoryColumn } from './components/CategoryColumn';

function App() {
  const [visible, setVisible] = useState(true);

  const { data: items = [] } = useQuery({
    queryKey: ['all-items'],
    queryFn: getAllItems,
    refetchOnWindowFocus: true,
  });

  // Group items by category
  const grouped = useMemo(() => {
    const map = new Map<number, { name: string; items: typeof items }>();
    for (const item of items) {
      let group = map.get(item.category_id);
      if (!group) {
        group = { name: item.category_name, items: [] };
        map.set(item.category_id, group);
      }
      group.items.push(item);
    }
    return Array.from(map.values());
  }, [items]);

  // Close overlay on Esc
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        setVisible(false);
        setTimeout(() => {
          getCurrentWebviewWindow().hide();
          setVisible(true);
        }, 200);
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, []);

  return (
    <div className="h-screen w-screen p-2">
      <div
        className={`flex h-full w-full rounded-xl bg-[#ececec] shadow-lg transition-opacity duration-200 ${
          visible ? 'opacity-100' : 'opacity-0'
        }`}
      >
        {grouped.length === 0 ? (
          <div className="flex h-full w-full items-center justify-center">
            <p className="text-sm text-gray-400">
              No items yet. Add categories and items in the main window.
            </p>
          </div>
        ) : (
          <div className="flex h-full items-start gap-5 overflow-x-auto overflow-y-auto p-6">
            {grouped.map((group) => (
              <CategoryColumn
                key={group.name}
                categoryName={group.name}
                items={group.items}
              />
            ))}
          </div>
        )}
      </div>
    </div>
  );
}

export default App;
