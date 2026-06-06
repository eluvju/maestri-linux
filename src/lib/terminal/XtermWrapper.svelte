<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { Terminal } from 'xterm';
  import { FitAddon } from 'xterm-addon-fit';
  import { startTerminalRead, writeToTerminal, resizeTerminal } from './terminal';
  import 'xterm/css/xterm.css';

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

    term.onData((data) => {
      writeToTerminal(nodeId, data);
    });

    startTerminalRead(nodeId, (data) => {
      term.write(data);
    }).then((fn) => { unlisten = fn; });

    requestAnimationFrame(() => {
      fitAddon.fit();
    });

    const resizeObserver = new ResizeObserver(() => {
      try {
        fitAddon.fit();
        const dims = fitAddon.proposeDimensions();
        if (dims) {
          resizeTerminal(nodeId, dims.cols, dims.rows, dims.width * 10, dims.height * 18);
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
