/**
 * Type-Safe IPC Client
 *
 * Provides compile-time type checking for Tauri invoke calls.
 * Maps command names to their argument and return types.
 */

import { invoke } from '@tauri-apps/api/core';
import type { CommandArgs, CommandReturns } from './ipc.generated';
export type { CommandArgs, CommandReturns } from './ipc.generated';

export type CommandName = keyof CommandArgs;
export type CommandArg<C extends CommandName> = CommandArgs[C];
export type CommandReturn<C extends CommandName> = CommandReturns[C];

/**
 * Type-safe Tauri invoke wrapper
 *
 * @example
 * ```typescript
 * const info = await typedInvoke("get_app_info", {});
 * const settings = await typedInvoke("get_settings", {});
 * await typedInvoke("set_settings", { input: { locale: "zh-CN", launch_on_login: null, theme: null } });
 * ```
 */
export async function typedInvoke<C extends CommandName>(
  command: C,
  args: CommandArg<C extends keyof CommandArgs ? C : never>
): Promise<CommandReturn<C extends keyof CommandReturns ? C : never>> {
  return invoke(command, args as Record<string, unknown>);
}

/**
 * Legacy wrapper for backward compatibility
 *
 * @deprecated Use `typedInvoke` for new code
 */
export async function tauriInvoke<T>(
  cmd: string,
  args?: Record<string, unknown>
): Promise<T> {
  return invoke<T>(cmd, args);
}
