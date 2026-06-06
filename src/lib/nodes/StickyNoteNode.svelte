<script lang="ts">
  import { onMount } from 'svelte';
  import { viewport } from '$lib/canvas/canvasStore';
  import { nodes } from './nodesStore';

  let {
    id,
    label = '',
    color = '#00ff7f',
    x,
    y,
    width = 480,
    height = 320,
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

  let showGroups = $state(false);

  let dragging = $state(false);
  let dragStartX = $state(0);
  let dragStartY = $state(0);
  let nodeStartX = $state(0);
  let nodeStartY = $state(0);
  let nodeX = $state(x);
  let nodeY = $state(y);
  let nodeW = $state(width);
  let nodeH = $state(height);
  let content = $state('');
  let editing = $state(false);
  let editContent = $state('');

  let resizing = $state(false);
  let resizeStartX = $state(0);
  let resizeStartY = $state(0);
  let resizeStartW = $state(0);
  let resizeStartH = $state(0);

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

  onMount(async () => {
    try {
      content = await nodes.getNoteContent(id);
      editContent = content;
    } catch {}
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
      nodeW = Math.max(200, resizeStartW + dx);
      nodeH = Math.max(100, resizeStartH + dy);
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

  async function startEdit() {
    editContent = content;
    editing = true;
  }

  async function saveEdit() {
    content = editContent;
    editing = false;
    await nodes.setNoteContent(id, content);
  }
</script>

<svelte:window onmousemove={handleMove} onmouseup={handleUp} />

<div
  class="note-node"
  style="
    left: {nodeX}px;
    top: {nodeY}px;
    width: {nodeW}px;
    height: {nodeH}px;
    border-color: {color};
  "
>
  <div class="note-header" onmousedown={handleHeaderDown}>
    <div class="note-dot" style="background: {color};" />
    <span class="note-label">{label || 'Note'}</span>
    <div class="note-actions">
      {#if groups.length > 0}
        <div class="group-badge-note" style="position:relative;" onclick={() => showGroups = !showGroups} role="button" tabindex="0">
          <span style="font-size:11px;">⊞</span>
          {#if showGroups}
            <div class="group-dropdown-note" onclick={(e) => e.stopPropagation()}>
              <button class="group-opt-note" onclick={() => { ongroupchange?.(null); showGroups = false; }}>— None</button>
              {#each groups as g}
                <button
                  class="group-opt-note"
                  style={groupId === g.id ? 'background:#0a0a0a;' : ''}
                  onclick={() => { ongroupchange?.(g.id); showGroups = false; }}
                >
                  <span class="group-dot-sm-note" style="background:{g.color};" />
                  {g.label || 'Group'}
                </button>
              {/each}
            </div>
          {/if}
        </div>
      {/if}
      <button class="note-btn" onclick={startEdit} title="Edit note">✎</button>
      <button class="note-btn close" onclick={(e) => { e.stopPropagation(); nodes.remove(id); }}>×</button>
    </div>
  </div>
  <div class="note-body">
    {#if editing}
      <div class="note-editor">
        <textarea
          class="note-textarea"
          bind:value={editContent}
          spellcheck="false"
        />
        <button class="save-btn" onclick={saveEdit}>Save</button>
      </div>
    {:else}
      <pre class="note-content">{content || '# Empty note'}</pre>
    {/if}
  </div>
  <div class="resize-handle" onmousedown={handleResizeDown} />
</div>

<style>
  .note-node {
    position: absolute;
    background: #0a0a0a;
    border: 1px solid #00ff7f;
    border-radius: 8px;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    box-shadow: 0 0 20px rgba(0, 255, 127, 0.1);
  }
  .note-header {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 4px 8px;
    background: #111;
    border-bottom: 1px solid #00ff7f;
    cursor: grab;
    user-select: none;
    height: 30px;
  }
  .note-header:active {
    cursor: grabbing;
  }
  .note-dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    flex-shrink: 0;
  }
  .note-label {
    flex: 1;
    font-size: 12px;
    color: #00ff7f;
    font-family: 'JetBrains Mono', 'Fira Code', monospace;
  }
  .note-actions {
    display: flex;
    gap: 4px;
  }
  .note-btn {
    background: none;
    border: none;
    color: #555;
    cursor: pointer;
    font-size: 14px;
    padding: 0 4px;
    line-height: 1;
  }
  .note-btn:hover {
    color: #00ff7f;
  }
  .note-btn.close:hover {
    color: #ff5555;
  }
  .note-body {
    flex: 1;
    overflow: auto;
    padding: 12px;
  }
  .note-content {
    margin: 0;
    color: #00ff7f;
    font-family: 'JetBrains Mono', 'Fira Code', monospace;
    font-size: 13px;
    line-height: 1.5;
    white-space: pre-wrap;
    word-wrap: break-word;
  }
  .note-editor {
    display: flex;
    flex-direction: column;
    gap: 8px;
    height: 100%;
  }
  .note-textarea {
    flex: 1;
    background: #000;
    border: 1px solid #333;
    color: #00ff7f;
    font-family: 'JetBrains Mono', 'Fira Code', monospace;
    font-size: 13px;
    padding: 8px;
    resize: none;
    outline: none;
    border-radius: 4px;
  }
  .note-textarea:focus {
    border-color: #00ff7f;
  }
  .save-btn {
    align-self: flex-end;
    background: #00ff7f;
    color: #000;
    border: none;
    padding: 4px 16px;
    border-radius: 4px;
    cursor: pointer;
    font-size: 12px;
    font-weight: 600;
  }
  .save-btn:hover {
    background: #00cc66;
  }
  .group-badge-note {
    display: flex;
    align-items: center;
    cursor: pointer;
    padding: 0 4px;
    color: #555;
    font-size: 12px;
  }
  .group-badge-note:hover {
    color: #00ff7f;
  }
  .group-dropdown-note {
    position: absolute;
    top: 24px;
    left: 0;
    min-width: 120px;
    background: #111;
    border: 1px solid #333;
    border-radius: 6px;
    z-index: 100;
    box-shadow: 0 4px 20px rgba(0,0,0,0.6);
    overflow: hidden;
  }
  .group-opt-note {
    display: flex;
    align-items: center;
    gap: 6px;
    width: 100%;
    padding: 6px 10px;
    background: none;
    border: none;
    color: #888;
    cursor: pointer;
    font-size: 12px;
    text-align: left;
  }
  .group-opt-note:hover {
    background: #0a0a0a;
    color: #00ff7f;
  }
  .group-dot-sm-note {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    flex-shrink: 0;
  }
  .resize-handle {
    position: absolute;
    bottom: 0;
    right: 0;
    width: 16px;
    height: 16px;
    cursor: nwse-resize;
    background: linear-gradient(135deg, transparent 50%, #00ff7f 50%);
    opacity: 0.5;
  }
  .resize-handle:hover {
    opacity: 1;
  }
</style>
