<script lang="ts">
  import { viewport } from '$lib/canvas/canvasStore';
  import { groups } from '$lib/nodes/nodesStore';

  let {
    id,
    label = 'Group',
    color = '#7c4dff',
    children,
    ondragend,
  }: {
    id: string;
    label?: string;
    color?: string;
    children?: any[];
    ondragend?: () => void;
  } = $props();

  let dragging = $state(false);
  let dragStartX = $state(0);
  let dragStartY = $state(0);
  let editing = $state(false);
  let editLabel = $state(label);
  let nodeLabel = $state(label);

  if (!children) children = [];

  function minChildX() {
    if (!children || children.length === 0) return 0;
    return Math.min(...children.map((c: any) => c.x));
  }

  function minChildY() {
    if (!children || children.length === 0) return 0;
    return Math.min(...children.map((c: any) => c.y));
  }

  function maxChildRight() {
    if (!children || children.length === 0) return 400;
    return Math.max(...children.map((c: any) => c.x + (c.width || 640)));
  }

  function maxChildBottom() {
    if (!children || children.length === 0) return 300;
    return Math.max(...children.map((c: any) => c.y + (c.height || 400)));
  }

  let gx = $derived(minChildX() - 16);
  let gy = $derived(minChildY() - 44);
  let gw = $derived(maxChildRight() - minChildX() + 32);
  let gh = $derived(maxChildBottom() - minChildY() + 32);

  function handleHeaderDown(e: MouseEvent) {
    if (e.button !== 0) return;
    dragging = true;
    dragStartX = e.clientX;
    dragStartY = e.clientY;
    e.preventDefault();
    e.stopPropagation();
  }

  function handleMove(e: MouseEvent) {
    if (!dragging) return;
    const dx = (e.clientX - dragStartX) / $viewport.zoom;
    const dy = (e.clientY - dragStartY) / $viewport.zoom;
    dragStartX = e.clientX;
    dragStartY = e.clientY;
    groups.move(id, dx, dy);
  }

  function handleUp() {
    dragging = false;
    ondragend?.();
  }

  function commitLabel() {
    nodeLabel = editLabel;
    editing = false;
    groups.setLabel(id, editLabel);
  }
</script>

<svelte:window onmousemove={handleMove} onmouseup={handleUp} />

<div
  class="group-box"
  style="
    left: {gx}px;
    top: {gy}px;
    width: {gw}px;
    height: {gh}px;
    border-color: {color};
  "
>
  <div class="group-header" onmousedown={handleHeaderDown} data-group-header>
    <div class="group-dot" style="background: {color};" />
    {#if editing}
      <input
        type="text"
        bind:value={editLabel}
        onblur={commitLabel}
        onkeydown={(e) => { if (e.key === 'Enter') commitLabel(); }}
        autofocus
        class="group-label-input"
      />
    {:else}
      <span
        class="group-label"
        ondblclick={() => { editLabel = nodeLabel; editing = true; }}
      >
        {nodeLabel || 'Group'}
      </span>
    {/if}
    <span class="group-count">{children?.length || 0} nodes</span>
  </div>
</div>

<style>
  .group-box {
    position: absolute;
    border: 2px dashed #7c4dff;
    border-radius: 16px;
    background: rgba(124, 77, 255, 0.05);
    pointer-events: none;
    z-index: 0;
  }
  .group-header {
    position: absolute;
    top: -28px;
    left: 8px;
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 2px 10px;
    background: #16213e;
    border: 1px solid #333;
    border-radius: 8px;
    cursor: grab;
    pointer-events: auto;
    user-select: none;
    height: 28px;
  }
  .group-header:active {
    cursor: grabbing;
  }
  .group-dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    flex-shrink: 0;
  }
  .group-label {
    font-size: 12px;
    color: #a0a0c0;
    font-weight: 600;
  }
  .group-label-input {
    background: #0f3460;
    border: 1px solid #e94560;
    color: #e0e0e0;
    padding: 1px 6px;
    border-radius: 4px;
    font-size: 12px;
    outline: none;
    width: 120px;
  }
  .group-count {
    font-size: 10px;
    color: #666;
    margin-left: 4px;
  }
</style>
