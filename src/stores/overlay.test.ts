import { describe, expect, it } from 'vitest';
import { createStore } from 'jotai';
import { overlayVisibleAtom } from './overlay';

describe('overlay store', () => {
  it('overlay is hidden by default', () => {
    const store = createStore();
    expect(store.get(overlayVisibleAtom)).toBe(false);
  });

  it('allows toggling visibility', () => {
    const store = createStore();
    store.set(overlayVisibleAtom, true);
    expect(store.get(overlayVisibleAtom)).toBe(true);
  });
});
