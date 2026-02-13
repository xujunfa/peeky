import '@testing-library/jest-dom';
import { vi } from 'vitest';

// Mock Tauri APIs
const mockInvoke = vi.fn();
vi.mock('@tauri-apps/api/core', () => ({
  invoke: mockInvoke,
}));

// Mock Tauri event listener
vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn(() => Promise.resolve(() => {})),
  emit: vi.fn(),
}));

// Export mock for use in tests
export { mockInvoke };
