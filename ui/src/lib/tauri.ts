import { invoke, type InvokeArgs } from "@tauri-apps/api/core";

export async function invokeSafe<T>(command: string, payload?: InvokeArgs): Promise<T> {
  if (!("__TAURI_INTERNALS__" in window)) {
    throw new Error("Tauri runtime unavailable (browser/test mode)");
  }
  return invoke<T>(command, payload);
}
