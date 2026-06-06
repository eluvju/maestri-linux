import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { UnlistenFn } from '@tauri-apps/api/event';

function log(level: string, msg: string) {
  invoke('log_msg', { level, msg }).catch(() => {});
}

export function startTerminalRead(nodeId: string, onData: (data: Uint8Array) => void): Promise<UnlistenFn> {
  log('terminal', 'startTerminalRead nodeId=' + nodeId);

  // Timeout to detect if listen hangs
  setTimeout(() => {
    log('terminal', 'TIMEOUT: listener still pending after 5s nodeId=' + nodeId);
  }, 5000);

  // Register listener (don't await — might hang in some Tauri v2 versions)
  const listenPromise = listen<number[]>(`pty-output-${nodeId}`, (event) => {
    log('terminal', 'event received bytes=' + event.payload.length);
    onData(new Uint8Array(event.payload));
  });

  // Immediately get buffered data (parallel, not serial)
  invoke<number[]>('get_pty_buffer', { id: nodeId }).then(buf => {
    if (buf.length > 0) {
      log('terminal', 'got buffer bytes=' + buf.length);
      onData(new Uint8Array(buf));
    } else {
      log('terminal', 'no buffer for node');
    }
  }).catch(e => log('terminal', 'buffer error: ' + e));

  // Poll buffer every 500ms as fallback (in case listen never fires)
  let stopped = false;
  const pollInterval = setInterval(() => {
    if (stopped) { clearInterval(pollInterval); return; }
    invoke<number[]>('get_pty_buffer', { id: nodeId }).then(buf => {
      if (buf.length > 0) {
        log('terminal', 'poll got buffer bytes=' + buf.length);
        onData(new Uint8Array(buf));
        clearInterval(pollInterval);
      }
    }).catch(() => {});
  }, 500);

  return listenPromise.then(fn => {
    stopped = true;
    clearInterval(pollInterval);
    log('terminal', 'listener registered');
    return fn;
  }).catch(e => {
    stopped = true;
    clearInterval(pollInterval);
    log('terminal', 'ERROR in listen: ' + e);
    return (() => {}) as UnlistenFn;
  });
}

export async function writeToTerminal(nodeId: string, data: string) {
  log('terminal', 'writeToTerminal bytes=' + data.length);
  await invoke('write_pty', { id: nodeId, data: Array.from(new TextEncoder().encode(data)) });
}

export async function resizeTerminal(nodeId: string, cols: number, rows: number, width: number, height: number) {
  log('terminal', 'resize cols=' + cols + ' rows=' + rows);
  await invoke('resize_pty', { id: nodeId, cols, rows, width, height });
}
