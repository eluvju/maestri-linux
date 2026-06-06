<script lang="ts">
  import Connection from './Connection.svelte';
  import { connections, nodes } from '$lib/nodes/nodesStore';

  function getNodeCenter(nodeId: string): { x: number; y: number } | null {
    const node = $nodes.find((n) => n.id === nodeId);
    if (!node) return null;
    return {
      x: node.x + node.width / 2,
      y: node.y + node.height / 2,
    };
  }

  let pairs = $derived(
    $connections
      .map((c) => {
        const src = getNodeCenter(c.source_id);
        const tgt = getNodeCenter(c.target_id);
        if (!src || !tgt) return null;
        return { id: c.id, x1: src.x, y1: src.y, x2: tgt.x, y2: tgt.y };
      })
      .filter(Boolean)
  );
</script>

<svg
  class="connection-layer"
  width="100%"
  height="100%"
>
  {#each pairs as p (p.id)}
    <Connection x1={p.x1} y1={p.y1} x2={p.x2} y2={p.y2} />
  {/each}
</svg>

<style>
  .connection-layer {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    pointer-events: none;
    z-index: 0;
  }
</style>
