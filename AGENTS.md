# Build
npm run tauri build
Cargo path: ~/.cargo/bin/cargo
AppImage: src-tauri/target/release/bundle/appimage/Maestri\ Linux_0.1.0_amd64.AppImage
Symlink: ~/.local/bin/maestri-linux

# Debug sem DevTools
Log em arquivo: /tmp/maestri-linux.log
Comando: `rm -f /tmp/maestri-linux.log && maestri-linux & sleep 5 && cat /tmp/maestri-linux.log`

# Comandos de log
- `log_msg(level, msg)` no Rust → /tmp/maestri-linux.log
- `log_to_file(msg)` no Rust → /tmp/maestri-linux.log
- Frontend ts: `invoke('log_msg', { level, msg })` → /tmp/maestri-linux.log

# Arquivos chave
- src-tauri/src/lib.rs: create/remove/move node, labels, colors, groups, save/load, log_msg
- src-tauri/src/state.rs: Graph (nodes, connections, groups), Node (service, group_id), Group, save/load
- src-tauri/src/pty.rs: PtyManager::spawn (accepts service string, reader thread → emitter + buffer)
- src/lib/terminal/terminal.ts: startTerminalRead (listen + buffer replay)
- src/lib/terminal/XtermWrapper.svelte: xterm.js init + listener registration
- src/lib/nodes/nodesStore.ts: stores (nodes, connections, groups) with create/remove/move/setLabel/setColor
- src/lib/nodes/NodeHeader.svelte: inline edit + color picker, onlabelchange callback
- src/lib/nodes/TerminalNode.svelte: drag logic, delete handler, renders XtermWrapper
- src/lib/nodes/GroupNode.svelte: dashed border container, drag header to move group, double-click rename
- src/lib/modals/CreateNodeModal.svelte: service selector modal (shell/openclaw/opencode/codex) with ASCII art
- src/lib/toolbar/Toolbar.svelte: +Terminal, +Group, Connect, Delete buttons
- src/App.svelte: canvas, toolbar, modal, group rendering, refreshGraph wiring
- src/lib/canvas/Canvas.svelte: `<slot />` (Svelte 5 deprecated but works), double-click → canvas-dblclick event

# Event flow
1. double-click → App.svelte addNode → showModal=true → CreateNodeModal
2. service select → nodes.create(x, y, service) → invoke('create_node', { x, y, service })
3. Rust create_node: add_node to graph → spawn(ptys, service, emitter) → spawn returns
4. PTY reader thread: reads from fd → emitter(data) → app.emit('pty-output-{id}')
     → also stores in Arc<Mutex<Vec<u8>>> buffer
5. Frontend: refreshGraph() → svelte renders TerminalNode → XtermWrapper mounts
6. startTerminalRead: listen('pty-output-{id}') → get_pty_buffer → term.write(buf)

# Tests
cargo test (src-tauri)
3 tests: test_pty_spawn_and_buffer, test_graph_operations, test_group_operations

# Features implementadas
- Terminal nodes com PTY (xterm.js + Rust forkpty)
- Arrastar/redimensionar nodes no canvas
- Editar label inline (double-click no nome)
- Paleta de cores (12 cores) no NodeHeader
- Service selector modal com ASCII art (shell, opencode, openclaw, codex)
- Groups: container visual (dashed border + header), arrastar move children, renomear
- Board persistence: auto-save em ~/.local/share/maestri-linux/board.json
