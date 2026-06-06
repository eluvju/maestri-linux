<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { Terminal } from 'xterm';
  import { FitAddon } from 'xterm-addon-fit';
  import { invoke } from '@tauri-apps/api/core';
  import { startTerminalRead, writeToTerminal, resizeTerminal } from './terminal';
  import 'xterm/css/xterm.css';

  function log(msg: string) {
    invoke('log_msg', { level: 'XtermWrapper'.toString(), msg: msg + ' nodeId=' + nodeId }).catch(() => {});
  }

  let {
    nodeId,
  }: {
    nodeId: string;
  } = $props();

  let container: HTMLDivElement;
  let term: Terminal;
  let fitAddon: FitAddon;
  let unlisten: (() => void) | null = null;

  onMount(() => {
    log('mounting');
    term = new Terminal({
      cursorBlink: true,
      cursorStyle: 'block',
      fontSize: 14,
      fontFamily: "'JetBrains Mono', 'Fira Code', monospace",
      theme: {
        background: '#1a1a2e',
        foreground: '#e0e0e0',
        cursor: '#e94560',
        selectionBackground: '#0f3460',
        black: '#1a1a2e',
        red: '#e94560',
        green: '#00ff7f',
        yellow: '#ffd700',
        blue: '#00bfff',
        magenta: '#ff69b4',
        cyan: '#00ffff',
        white: '#e0e0e0',
      },
      allowTransparency: true,
    });

    fitAddon = new FitAddon();
    term.loadAddon(fitAddon);
    term.open(container);
    log('terminal opened');

    term.onData((data) => {
      log('onData length=' + data.length);
      writeToTerminal(nodeId, data);
    });

    startTerminalRead(nodeId, (data) => {
      log('got output bytes=' + data.length);
      term.write(data);
    }).then((fn) => { unlisten = fn; log('listener registered'); });

    requestAnimationFrame(() => {
      fitAddon.fit();
      log('fitAddon.fit() done');
    });

    const resizeObserver = new ResizeObserver(() => {
      try {
        fitAddon.fit();
        const dims = fitAddon.proposeDimensions();
        if (dims) {
          resizeTerminal(nodeId, dims.cols, dims.rows, 640, 400);
        }
      } catch {}
    });
    resizeObserver.observe(container);

    return () => {
      resizeObserver.disconnect();
    };
  });

  onDestroy(() => {
    if (term) term.dispose();
    if (unlisten) unlisten();
  });
</script>

<div bind:this={container} class="xterm-wrapper"></div>

<style>
  .xterm-wrapper {
    height: 100%;
    width: 100%;
  }
</style>
