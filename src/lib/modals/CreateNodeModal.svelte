<script lang="ts">
  let {
    onselect,
    onclose,
  }: {
    onselect: (service: string, command: string) => void;
    onclose: () => void;
  } = $props();

  const services = [
    {
      id: 'shell',
      name: 'Shell',
      command: '',
      desc: 'Terminal bash padrão',
      ascii: `┌─┐┌─┐┌┬┐┬ ┬┌┐┌
└─┐├┤  │ │ ││││
└─┘└─┘ ┴ └─┘┘└┘`,
    },
    {
      id: 'opencode',
      name: 'OpenCode',
      command: 'opencode',
      desc: 'Agente AI para coding tasks',
      ascii: `  ___                   _        _
 / _ \\ _ __   ___   ___| | _____| |__
| | | | '_ \\ / _ \\ / __| |/ / _ \\ '_ \\
| |_| | |_) | (_) | (__|   <  __/ | | |
 \\___/| .__/ \\___/ \\___|_|\\_\\___|_| |_|
      |_|`,
    },
    {
      id: 'openclaw',
      name: 'OpenClaw',
      command: 'openclaw tui',
      desc: 'Infraestrutura e deploy',
      ascii: `  ___                   _      _
 / _ \\ _ __   ___ ___| | ___| |_ __
| | | | '_ \\ / __/ _ \\ |/ _ \\ | '_ \\
| |_| | |_) | (_|  __/ |  __/ | |_) |
 \\___/| .__/ \\___\\___|_|\\___|_| .__/
      |_|                     |_|`,
    },
    {
      id: 'codex',
      name: 'Codex CLI',
      command: 'codex',
      desc: 'AI assistente no terminal',
      ascii: `  ___          _          _
 / __\\___   __| | ___  __| |
/ /  / _ \\ / _` + '`' + ` |/ _ \\/ _` + '`' + ` |
/ /__| (_) | (_| |  __/ (_| |
\\____/\\___/ \\__,_|\\___|\\__,_|`,
    },
  ];

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') onclose();
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="backdrop" onclick={onclose} role="presentation">
  <div class="modal" onclick={(e) => e.stopPropagation()} role="dialog" aria-label="Choose service">
    <h2 class="title">New Terminal</h2>
    <p class="subtitle">Select the service to run inside the terminal</p>
    <div class="grid">
      {#each services as srv}
        <button class="card" onclick={() => onselect(srv.id, srv.command)}>
          <pre class="ascii">{srv.ascii}</pre>
          <span class="srv-name">{srv.name}</span>
          <span class="srv-desc">{srv.desc}</span>
        </button>
      {/each}
    </div>
    <button class="cancel" onclick={onclose}>Cancel</button>
  </div>
</div>

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    backdrop-filter: blur(4px);
  }
  .modal {
    background: #1a1a2e;
    border: 1px solid #333;
    border-radius: 16px;
    padding: 32px;
    max-width: 720px;
    width: 90%;
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
  }
  .title {
    margin: 0 0 4px;
    color: #e0e0e0;
    font-size: 22px;
    font-weight: 600;
  }
  .subtitle {
    margin: 0 0 24px;
    color: #888;
    font-size: 14px;
  }
  .grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 12px;
  }
  .card {
    background: #16213e;
    border: 1px solid #2a2a4a;
    border-radius: 12px;
    padding: 16px;
    cursor: pointer;
    text-align: center;
    transition: all 0.2s;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
  }
  .card:hover {
    border-color: #e94560;
    background: #1a2744;
    transform: translateY(-2px);
    box-shadow: 0 8px 24px rgba(233, 69, 96, 0.15);
  }
  .ascii {
    font-size: 10px;
    line-height: 1.2;
    color: #e94560;
    margin: 0;
    font-family: 'JetBrains Mono', 'Fira Code', monospace;
    white-space: pre;
  }
  .srv-name {
    font-size: 15px;
    font-weight: 600;
    color: #e0e0e0;
  }
  .srv-desc {
    font-size: 12px;
    color: #888;
  }
  .cancel {
    margin-top: 16px;
    width: 100%;
    padding: 10px;
    background: none;
    border: 1px solid #333;
    border-radius: 8px;
    color: #888;
    cursor: pointer;
    font-size: 13px;
  }
  .cancel:hover {
    border-color: #555;
    color: #e0e0e0;
  }
</style>
