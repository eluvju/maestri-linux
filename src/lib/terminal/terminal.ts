import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { UnlistenFn } from '@tauri-apps/api/event';

export async function startTerminalRead(nodeId: string, onData: (data: Uint8Array) => void): Promise<UnlistenFn> {
  console.log('[terminal] startTerminalRead for', nodeId);
  const unlisten = await listen<number[]>(`pty-output-${nodeId}`, (event) => {
    console.log('[terminal] event received for', nodeId, 'bytes:', event.payload.length);
    onData(new Uint8Array(event.payload));
  });
  console.log('[terminal] listener registered for', nodeId);
  const buf = await invoke<number[]>('get_pty_buffer', { id: nodeId });
  if (buf.length > 0) {
    console.log('[terminal] got buffer for', nodeId, 'bytes:', buf.length);
    onData(new Uint8Array(buf));
  } else {
    console.log('[terminal] no buffer for', nodeId);
  }
  return unlisten;
}

export async function writeToTerminal(nodeId: string, data: string) {
  console.log('[terminal] writeToTerminal', nodeId, 'bytes:', data.length);
  await invoke('write_pty', { id: nodeId, data: Array.from(new TextEncoder().encode(data)) });
}

export async function resizeTerminal(nodeId: string, cols: number, rows: number, width: number, height: number) {
  console.log('[terminal] resize', nodeId, cols, rows);
  await invoke('resize_pty', { id: nodeId, cols, rows, width, height });
}
