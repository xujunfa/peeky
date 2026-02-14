import { useState } from 'react';
import { Separator } from '@/components/ui/separator';
import { CategoryList } from './components/CategoryList';
import { ItemList } from './components/ItemList';
import type { Category } from '@/core/ipc.generated';

function App() {
  const [selectedCategory, setSelectedCategory] = useState<Category | null>(null);

  return (
    <div className="flex h-screen bg-gradient-to-br from-slate-50 via-stone-50 to-emerald-50 text-slate-900">
      {/* Left: Categories */}
      <div className="flex w-64 shrink-0 flex-col border-r border-slate-200 p-4">
        <CategoryList
          selectedId={selectedCategory?.id ?? null}
          onSelect={setSelectedCategory}
        />
      </div>

      <Separator orientation="vertical" />

      {/* Right: Items */}
      <div className="flex flex-1 flex-col p-4">
        {selectedCategory ? (
          <ItemList category={selectedCategory} />
        ) : (
          <div className="flex flex-1 items-center justify-center">
            <p className="text-sm text-slate-400">
              Select a category to manage its items
            </p>
          </div>
        )}
      </div>
    </div>
  );
}

export default App;
