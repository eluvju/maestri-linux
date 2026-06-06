<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import Canvas from '$lib/canvas/Canvas.svelte';
  import Toolbar from '$lib/toolbar/Toolbar.svelte';
  import TerminalNode from '$lib/nodes/TerminalNode.svelte';
  import ConnectionLayer from '$lib/connections/ConnectionLayer.svelte';
  import { nodes, connections } from '$lib/nodes/nodesStore';
  import { viewport } from '$lib/canvas/canvasStore';

  let activeNodeId = $state<string | null>(null);

  async function addNode(e: CustomEvent) {
    const pos = e.detail;
    await nodes.create(pos.x, pos.y);
    await refreshGraph();
  }

  async function addNodeCenter() {
    const pos = viewport.screenToCanvas($viewport, window.innerWidth / 2, window.innerHeight / 2);
    await nodes.create(pos.x, pos.y);
    await refreshGraph();
  }

  async function refreshGraph() {
    const graph = await invoke<any>('get_graph');
    nodes.refresh(graph.nodes);
    connections.refresh(graph.connections);
  }

  async function handleDelete() {
    if (activeNodeId) {
      await nodes.remove(activeNodeId);
      activeNodeId = null;
      await refreshGraph();
    } else if ($nodes.length > 0) {
      const last = $nodes[$nodes.length - 1];
      await nodes.remove(last.id);
      await refreshGraph();
    }
  }

  function selectNode(id: string) {
    activeNodeId = id;
  }

  onMount(refreshGraph);
</script>

<Toolbar
  onaddnode={addNodeCenter}
  ondelete={handleDelete}
  hasSelection={$nodes.length > 0}
/>

<Canvas oncanvas-dblclick={addNode}>
  <ConnectionLayer />
  {#each $nodes as node (node.id)}
    <TerminalNode
      id={node.id}
      label={node.label}
      color={node.color}
      x={node.x}
      y={node.y}
      width={node.width}
      height={node.height}
      onclick={() => selectNode(node.id)}
    />
  {/each}
</Canvas>

<style>
  :global(body) {
    margin: 0;
    overflow: hidden;
  }
</style>
