<script lang="ts">
  import { viewport } from '$lib/canvas/canvasStore';
  import NodeHeader from './NodeHeader.svelte';
  import { nodes } from './nodesStore';

  let {
    id,
    x,
    y,
    width = 640,
    height = 400,
  }: {
    id: string;
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

  function handleHeaderDown(e: MouseEvent) {
    dragging = true;
    dragStartX = e.clientX;
    dragStartY = e.clientY;
    nodeStartX = nodeX;
    nodeStartY = nodeY;
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

  async function handleDelete() {
    await nodes.remove(id);
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
  "
>
  <NodeHeader label="Terminal" ondelet={handleDelete} onmousedown={handleHeaderDown} />
  <div
    class="terminal-body"
    onmousedown={(e) => e.stopPropagation()}
    role="terminal"
  >
    <div class="xterm-container" id="xterm-{id}"></div>
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
