<script lang="ts">
  let {
    label = '',
    ondelete,
  }: {
    label?: string;
    ondelete?: () => void;
  } = $props();

  let editing = $state(false);
  let editLabel = $state(label);
  let nodeLabel = $state(label);
</script>

<div class="header" data-node-header>
  {#if editing}
    <input
      type="text"
      bind:value={editLabel}
      onblur={() => { nodeLabel = editLabel; editing = false; }}
      onkeydown={(e) => { if (e.key === 'Enter') { nodeLabel = editLabel; editing = false; } }}
      autofocus
    />
  {:else}
    <span class="label" ondblclick={() => { editLabel = nodeLabel; editing = true; }}>
      {nodeLabel || 'Terminal'}
    </span>
  {/if}
  <button class="close-btn" onclick={ondelete}>×</button>
</div>

<style>
  .header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 4px 8px;
    background: #16213e;
    border-bottom: 1px solid #0f3460;
    border-radius: 8px 8px 0 0;
    cursor: grab;
    user-select: none;
    height: 32px;
  }
  .header:active {
    cursor: grabbing;
  }
  .label {
    font-size: 13px;
    color: #a0a0c0;
    flex: 1;
  }
  .close-btn {
    background: none;
    border: none;
    color: #666;
    cursor: pointer;
    font-size: 16px;
    padding: 0 4px;
    line-height: 1;
  }
  .close-btn:hover {
    color: #ff5555;
  }
  input {
    background: #0f3460;
    border: 1px solid #e94560;
    color: #e0e0e0;
    padding: 2px 6px;
    border-radius: 4px;
    font-size: 13px;
    flex: 1;
    outline: none;
  }
</style>
