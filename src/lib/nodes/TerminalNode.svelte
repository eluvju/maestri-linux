<script lang="ts">
  import { viewport } from '$lib/canvas/canvasStore';
  import NodeHeader from './NodeHeader.svelte';
  import XtermWrapper from '$lib/terminal/XtermWrapper.svelte';
  import { nodes } from './nodesStore';

  let {
    id,
    label = '',
    color = '#0f3460',
    x,
    y,
    width = 640,
    height = 400,
    groupId = null,
    groups = [],
    ongroupchange,
  }: {
    id: string;
    label?: string;
    color?: string;
    x: number;
    y: number;
    width?: number;
    height?: number;
    groupId?: string | null;
    groups?: Array<{ id: string; label: string; color: string }>;
    ongroupchange?: (gid: string | null) => void;
  } = $props();

  let dragging = $state(false);
  let dragStartX = $state(0);
  let dragStartY = $state(0);
  let nodeStartX = $state(0);
  let nodeStartY = $state(0);
  let nodeX = $state(x);
  let nodeY = $state(y);

  let resizing = $state(false);
  let resizeStartX = $state(0);
  let resizeStartY = $state(0);
  let resizeStartW = $state(0);
  let resizeStartH = $state(0);
  let nodeW = $state(width);
  let nodeH = $state(height);

  $effect(() => {
    if (!dragging) {
      nodeX = x;
      nodeY = y;
    }
  });

  $effect(() => {
    nodeW = width;
    nodeH = height;
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
    if (dragging) {
      const dx = (e.clientX - dragStartX) / $viewport.zoom;
      const dy = (e.clientY - dragStartY) / $viewport.zoom;
      nodeX = nodeStartX + dx;
      nodeY = nodeStartY + dy;
    }
    if (resizing) {
      const dx = (e.clientX - resizeStartX) / $viewport.zoom;
      const dy = (e.clientY - resizeStartY) / $viewport.zoom;
      nodeW = Math.max(320, resizeStartW + dx);
      nodeH = Math.max(200, resizeStartH + dy);
    }
  }

  function handleUp() {
    if (dragging) {
      dragging = false;
      nodes.move(id, nodeX, nodeY);
    }
    if (resizing) {
      resizing = false;
      nodes.setSize(id, nodeW, nodeH);
    }
  }

  function handleResizeDown(e: MouseEvent) {
    e.preventDefault();
    e.stopPropagation();
    resizing = true;
    resizeStartX = e.clientX;
    resizeStartY = e.clientY;
    resizeStartW = nodeW;
    resizeStartH = nodeH;
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
    width: {nodeW}px;
    height: {nodeH}px;
    border-color: {color};
  "
>
  <NodeHeader {label} {color} {groupId} {groups} ondelete={handleDelete} onmousedown={handleHeaderDown} oncolorchange={(c) => nodes.setColor(id, c)} onlabelchange={(l) => nodes.setLabel(id, l)} ongroupchange={(gid) => ongroupchange?.(gid)} />
  <div
    class="terminal-body"
    onmousedown={(e) => e.stopPropagation()}
    role="terminal"
  >
    <XtermWrapper nodeId={id} />
  </div>
  <div class="resize-handle" onmousedown={handleResizeDown} />
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
  .resize-handle {
    position: absolute;
    bottom: 0;
    right: 0;
    width: 16px;
    height: 16px;
    cursor: nwse-resize;
    background: linear-gradient(135deg, transparent 50%, rgba(255,255,255,0.3) 50%);
    opacity: 0.5;
    z-index: 10;
  }
  .resize-handle:hover {
    opacity: 1;
  }
</style>
