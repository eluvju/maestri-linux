import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { UnlistenFn } from '@tauri-apps/api/event';

export async function startTerminalRead(nodeId: string, onData: (data: Uint8Array) => void): Promise<UnlistenFn> {
  const unlisten = await listen<number[]>(`pty-output-${nodeId}`, (event) => {
    onData(new Uint8Array(event.payload));
  });
  return unlisten;
}

export async function writeToTerminal(nodeId: string, data: string) {
  await invoke('write_pty', { id: nodeId, data: Array.from(new TextEncoder().encode(data)) });
}

export async function resizeTerminal(nodeId: string, cols: number, rows: number, width: number, height: number) {
  await invoke('resize_pty', { id: nodeId, cols, rows, width, height });
}
