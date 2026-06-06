<script lang="ts">
  let {
    onaddnode,
    ondelete,
    onconnect,
    onaddgroup,
    onaddnote,
    hasSelection = false,
  }: {
    onaddnode?: () => void;
    ondelete?: () => void;
    onconnect?: () => void;
    onaddgroup?: () => void;
    onaddnote?: () => void;
    hasSelection?: boolean;
  } = $props();
</script>

<div class="toolbar">
  <button class="tool-btn" onclick={onaddnode} title="Add terminal node">
    + Terminal
  </button>
  <button class="tool-btn" onclick={onaddnote} title="Add sticky note">
    + Note
  </button>
  <button class="tool-btn" onclick={onaddgroup} title="Add group">
    + Group
  </button>
  {#if hasSelection}
    <button class="tool-btn" onclick={onconnect} title="Connect selected nodes">
      Connect
    </button>
    <button class="tool-btn danger" onclick={ondelete} title="Delete selected">
      Delete
    </button>
  {/if}
</div>

<style>
  .toolbar {
    position: fixed;
    top: 16px;
    left: 50%;
    transform: translateX(-50%);
    display: flex;
    gap: 8px;
    z-index: 100;
    background: rgba(22, 33, 62, 0.9);
    padding: 8px 16px;
    border-radius: 12px;
    border: 1px solid #0f3460;
    backdrop-filter: blur(10px);
  }
  .tool-btn {
    background: #0f3460;
    color: #e0e0e0;
    border: 1px solid transparent;
    padding: 6px 14px;
    border-radius: 6px;
    cursor: pointer;
    font-size: 13px;
    font-family: inherit;
    transition: all 0.15s;
  }
  .tool-btn:hover {
    background: #e94560;
    color: #fff;
  }
  .tool-btn.danger:hover {
    background: #ff3333;
  }
</style>
