import { describe, expect, it } from 'vitest';
import { createStore } from 'jotai';
import { timerPanelVisibleAtom, timerPanelMessageAtom } from './timer';

describe('timer placeholder store', () => {
  it('has visible panel by default', () => {
    const store = createStore();
    expect(store.get(timerPanelVisibleAtom)).toBe(true);
  });

  it('allows message updates', () => {
    const store = createStore();
    store.set(timerPanelMessageAtom, 'Custom placeholder');
    expect(store.get(timerPanelMessageAtom)).toBe('Custom placeholder');
  });
});
