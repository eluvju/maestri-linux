<script lang="ts">
  import { viewport } from '$lib/canvas/canvasStore';
  import NodeHeader from './NodeHeader.svelte';
  import XtermWrapper from '$lib/terminal/XtermWrapper.svelte';
  import { nodes } from './nodesStore';

  let {
    id,
    label = 'Terminal',
    color = '#0f3460',
    x,
    y,
    width = 640,
    height = 400,
  }: {
    id: string;
    label?: string;
    color?: string;
    x: number;
    y: number;
    width?: number;
    height?: number;
  } = $props();

  let dragging = $state(false);
  let dragStartX = $state(0);
  let dragStartY = $state(0);
  let nodeStartX = $state(0);
  let nodeStartY = $state(0);
  let nodeX = $state(x);
  let nodeY = $state(y);

  $effect(() => {
    if (!dragging) {
      nodeX = x;
      nodeY = y;
    }
  });

  function handleHeaderDown(e: MouseEvent) {
    if (e.button !== 0) return;
    dragging = true;
    dragStartX = e.clientX;
    dragStartY = e.clientY;
    nodeStartX = nodeX;
    nodeStartY = nodeY;
    e.preventDefault();
    e.stopPropagation();
  }

  function handleMove(e: MouseEvent) {
    if (!dragging) return;
    const dx = (e.clientX - dragStartX) / $viewport.zoom;
    const dy = (e.clientY - dragStartY) / $viewport.zoom;
    nodeX = nodeStartX + dx;
    nodeY = nodeStartY + dy;
  }

  function handleUp() {
    if (dragging) {
      dragging = false;
      nodes.move(id, nodeX, nodeY);
    }
  }

  async function handleDelete(e: MouseEvent) {
    e.stopPropagation();
    await nodes.remove(id).catch(() => {});
  }
</script>

<svelte:window onmousemove={handleMove} onmouseup={handleUp} />

<div
  class="terminal-node"
  data-node="true"
  style="
    left: {nodeX}px;
    top: {nodeY}px;
    width: {width}px;
    height: {height}px;
    border-color: {color};
  "
>
  <NodeHeader {label} {color} ondelete={handleDelete} onmousedown={handleHeaderDown} oncolorchange={(c) => nodes.setColor(id, c)} />
  <div
    class="terminal-body"
    onmousedown={(e) => e.stopPropagation()}
    role="terminal"
  >
    <XtermWrapper nodeId={id} />
  </div>
</div>

<style>
  .terminal-node {
    position: absolute;
    background: #1a1a2e;
    border: 1px solid #0f3460;
    border-radius: 8px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.4);
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }
  .terminal-body {
    flex: 1;
    padding: 4px;
    overflow: hidden;
  }
  .xterm-container {
    height: 100%;
    width: 100%;
  }
</style>
