/**
 * Tauri invoke wrapper
 *
 * Re-exports from core/ipc.ts for centralized type definitions.
 */

export {
  typedInvoke,
  tauriInvoke,
  type CommandName,
  type CommandArg,
  type CommandReturn,
  type CommandArgs,
  type CommandReturns,
} from '@/core/ipc';
