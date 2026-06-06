<script lang="ts">
  import { viewport } from './canvasStore';
  import CanvasGrid from './CanvasGrid.svelte';

  let panning = $state(false);
  let lastX = $state(0);
  let lastY = $state(0);

  function handleMouseDown(e: MouseEvent) {
    if ((e.target as HTMLElement).closest('[data-node]')) return;
    panning = true;
    lastX = e.clientX;
    lastY = e.clientY;
  }

  function handleMouseMove(e: MouseEvent) {
    if (!panning) return;
    viewport.pan(e.clientX - lastX, e.clientY - lastY);
    lastX = e.clientX;
    lastY = e.clientY;
  }

  function handleMouseUp() {
    panning = false;
  }

  function handleWheel(e: WheelEvent) {
    e.preventDefault();
    const amount = -e.deltaY * 0.001;
    viewport.zoomAt(amount, e.clientX, e.clientY);
  }

  function handleDblClick(e: MouseEvent) {
    const pos = viewport.screenToCanvas($viewport, e.clientX, e.clientY);
    const event = new CustomEvent('canvas-dblclick', { detail: pos });
    (e.currentTarget as HTMLElement).dispatchEvent(event);
  }
</script>

<div
  class="canvas"
  onmousedown={handleMouseDown}
  onmousemove={handleMouseMove}
  onmouseup={handleMouseUp}
  onmouseleave={handleMouseUp}
  onwheel={handleWheel}
  ondblclick={handleDblClick}
  role="application"
  tabindex={0}
  style="cursor: {panning ? 'grabbing' : 'grab'}"
>
  <CanvasGrid />
  <div
    class="canvas-content"
    style="
      transform: translate({$viewport.x}px, {$viewport.y}px) scale({$viewport.zoom});
      transform-origin: 0 0;
    "
  >
    <slot />
  </div>
</div>

<style>
  .canvas {
    width: 100%;
    height: 100%;
    position: relative;
    overflow: hidden;
    background: #1a1a2e;
    outline: none;
  }
  .canvas-content {
    position: absolute;
    top: 0;
    left: 0;
  }
</style>
