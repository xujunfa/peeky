import { describe, it, expect } from 'vitest';
import {
  TEMPLATE_INFO,
  DEFAULT_WINDOW_LABELS,
  DEFAULT_SHORTCUTS,
} from './config';

describe('template config', () => {
  it('exposes app metadata', () => {
    expect(TEMPLATE_INFO.name).toBe('Peeky');
    expect(TEMPLATE_INFO.version).toBe('0.1.0');
    expect(TEMPLATE_INFO.description.length).toBeGreaterThan(0);
  });

  it('defines default window labels', () => {
    expect(DEFAULT_WINDOW_LABELS.main).toBe('main');
    expect(DEFAULT_WINDOW_LABELS.overlay).toBe('overlay');
  });

  it('defines default shortcuts', () => {
    expect(DEFAULT_SHORTCUTS.toggleOverlay).toBe('Cmd+Shift+O');
    expect(DEFAULT_SHORTCUTS.toggleMain).toBe('Cmd+Shift+L');
  });
});
