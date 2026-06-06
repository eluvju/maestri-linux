<script lang="ts">
  let {
    label = '',
    color = '#0f3460',
    ondelete,
    oncolorchange,
  }: {
    label?: string;
    color?: string;
    ondelete?: () => void;
    oncolorchange?: (c: string) => void;
  } = $props();

  let editing = $state(false);
  let editLabel = $state(label);
  let nodeLabel = $state(label);
  let showColors = $state(false);

  const palette = ['#e94560','#ff6b6b','#ffd700','#00ff7f','#00bfff','#7c4dff','#ff69b4','#ff8c00','#0f3460','#2d3436','#636e72','#b2bec3'];

  function selectColor(c: string) {
    oncolorchange?.(c);
    showColors = false;
  }
</script>

<div class="header" data-node-header style="border-bottom-color: {color};">
  <div class="color-dot" style="background: {color};" onclick={() => showColors = !showColors} role="button" tabindex="0">
    {#if showColors}
      <div class="color-picker" onclick={(e) => e.stopPropagation()}>
        {#each palette as c}
          <button class="color-swatch" style="background:{c};" onclick={() => selectColor(c)} aria-label={c}></button>
        {/each}
      </div>
    {/if}
  </div>
  {#if editing}
    <input
      type="text"
      bind:value={editLabel}
      onblur={() => { nodeLabel = editLabel; editing = false; }}
      onkeydown={(e) => { if (e.key === 'Enter') { nodeLabel = editLabel; editing = false; } }}
      autofocus
      style="background:#0f3460;border:1px solid #e94560;color:#e0e0e0;padding:2px 6px;border-radius:4px;font-size:13px;flex:1;outline:none;"
    />
  {:else}
    <span class="label" role="button" tabindex="0" ondblclick={() => { editLabel = nodeLabel; editing = true; }}>
      {nodeLabel || 'Terminal'}
    </span>
  {/if}
  <button class="close-btn" onclick={ondelete}>×</button>
</div>

<style>
  .header {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 4px 8px;
    background: #16213e;
    border-bottom: 2px solid #0f3460;
    border-radius: 8px 8px 0 0;
    cursor: grab;
    user-select: none;
    height: 32px;
    position: relative;
  }
  .header:active {
    cursor: grabbing;
  }
  .color-dot {
    width: 12px;
    height: 12px;
    border-radius: 50%;
    cursor: pointer;
    flex-shrink: 0;
    position: relative;
  }
  .color-picker {
    position: absolute;
    top: 20px;
    left: 0;
    display: grid;
    grid-template-columns: repeat(6, 1fr);
    gap: 4px;
    padding: 6px;
    background: #16213e;
    border: 1px solid #333;
    border-radius: 8px;
    z-index: 100;
    box-shadow: 0 4px 20px rgba(0,0,0,0.6);
  }
  .color-swatch {
    width: 20px;
    height: 20px;
    border-radius: 4px;
    border: 1px solid rgba(255,255,255,0.1);
    cursor: pointer;
    padding: 0;
  }
  .color-swatch:hover {
    transform: scale(1.2);
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
</style>
