<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import Canvas from '$lib/canvas/Canvas.svelte';
  import Toolbar from '$lib/toolbar/Toolbar.svelte';
  import TerminalNode from '$lib/nodes/TerminalNode.svelte';
  import StickyNoteNode from '$lib/nodes/StickyNoteNode.svelte';
  import GroupNode from '$lib/nodes/GroupNode.svelte';
  import ConnectionLayer from '$lib/connections/ConnectionLayer.svelte';
  import CreateNodeModal from '$lib/modals/CreateNodeModal.svelte';
  import { nodes, connections, groups, assignToGroup } from '$lib/nodes/nodesStore';
  import { viewport } from '$lib/canvas/canvasStore';

  let activeNodeId = $state<string | null>(null);
  let showModal = $state(false);
  let pendingPos = $state<{ x: number; y: number } | null>(null);

  async function addNode(e: CustomEvent) {
    const pos = e.detail;
    pendingPos = pos;
    showModal = true;
  }

  async function handleServiceSelect(service: string, command: string) {
    showModal = false;
    if (!pendingPos) return;
    await nodes.create(pendingPos.x, pendingPos.y, service, command);
    pendingPos = null;
    await refreshGraph();
  }

  function closeModal() {
    showModal = false;
    pendingPos = null;
  }

  async function addNodeCenter() {
    const pos = viewport.screenToCanvas($viewport, window.innerWidth / 2, window.innerHeight / 2);
    pendingPos = pos;
    showModal = true;
  }

  async function addNote() {
    const pos = viewport.screenToCanvas($viewport, window.innerWidth / 2, window.innerHeight / 2);
    const id = await nodes.createNote(pos.x, pos.y, '# Nota\n\nEscreva seu lembrete aqui...');
    await refreshGraph();
  }

  async function addGroup() {
    const pos = viewport.screenToCanvas($viewport, window.innerWidth / 2, window.innerHeight / 2);
    const gid = await groups.create();
    await refreshGraph();
  }

  async function refreshGraph() {
    const graph = await invoke<any>('get_graph');
    nodes.refresh(graph.nodes);
    connections.refresh(graph.connections);
    groups.refresh(graph.groups);
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

  function getNodeById(id: string) {
    return $nodes.find(n => n.id === id);
  }

  onMount(refreshGraph);
</script>

<Toolbar
  onaddnode={addNodeCenter}
  onaddgroup={addGroup}
  onaddnote={addNote}
  ondelete={handleDelete}
  hasSelection={$nodes.length > 0}
/>

<Canvas oncanvas-dblclick={addNode}>
  <ConnectionLayer />
  {#each $groups as group (group.id)}
    <GroupNode
      id={group.id}
      label={group.label}
      color={group.color}
      children={group.child_ids.map(id => getNodeById(id)).filter(Boolean)}
      ondragend={refreshGraph}
    />
  {/each}
  {#each $nodes as node (node.id)}
    {#if node.kind === 'StickyNote'}
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <StickyNoteNode
        id={node.id}
        label={node.label}
        color={node.color}
        x={node.x}
        y={node.y}
        width={node.width}
        height={node.height}
        groupId={node.group_id}
        groups={$groups}
        ongroupchange={(gid) => { assignToGroup(node.id, gid); refreshGraph(); }}
        onclick={() => selectNode(node.id)}
      />
    {:else}
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <TerminalNode
        id={node.id}
        label={node.label}
        color={node.color}
        x={node.x}
        y={node.y}
        width={node.width}
        height={node.height}
        groupId={node.group_id}
        groups={$groups}
        ongroupchange={(gid) => { assignToGroup(node.id, gid); refreshGraph(); }}
        onclick={() => selectNode(node.id)}
      />
    {/if}
  {/each}
</Canvas>

{#if showModal}
  <CreateNodeModal onselect={handleServiceSelect} onclose={closeModal} />
{/if}

<style>
  :global(body) {
    margin: 0;
    overflow: hidden;
  }
</style>
