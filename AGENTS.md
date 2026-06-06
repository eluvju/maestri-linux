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
- src-tauri/src/lib.rs: todos os Tauri commands (create/remove/move/set_label/set_color/set_size nodes, groups CRUD, sticky notes, pty, log_msg)
- src-tauri/src/state.rs: Graph (Node com kind/service/group_id/width/height), Group, save/load, notes_dir
- src-tauri/src/pty.rs: PtyManager::spawn (sempre spawna shell, callback + buffer), write()
- src/lib/terminal/terminal.ts: startTerminalRead (listen + buffer replay em paralelo)
- src/lib/terminal/XtermWrapper.svelte: xterm.js init + listener registration
- src/lib/nodes/nodesStore.ts: stores (nodes, connections, groups) + assignToGroup(), getNoteContent(), setNoteContent()
- src/lib/nodes/NodeHeader.svelte: inline edit, color picker (12 cores), group picker dropdown, onmousedown para drag
- src/lib/nodes/TerminalNode.svelte: drag, resize handle (canto inferior direito), XtermWrapper, groupId/groups/ongroupchange
- src/lib/nodes/StickyNoteNode.svelte: nota markdown com aparência terminal monospace, editor inline, resize, group badge
- src/lib/nodes/GroupNode.svelte: container dashed border, drag header move children, double-click rename
- src/lib/modals/CreateNodeModal.svelte: seletor de serviço com ASCII art (shell, opencode, openclaw, codex), passa service + command
- src/lib/toolbar/Toolbar.svelte: +Terminal, +Note, +Group, Connect, Delete
- src/App.svelte: canvas, toolbar, modal, rendering condicional (TerminalNode vs StickyNoteNode), refreshGraph, assignToGroup
- src/lib/canvas/Canvas.svelte: `<slot />` (Svelte 5 deprecated mas funciona), double-click → canvas-dblclick event

# Event flow
1. double-click → App.svelte addNode → showModal=true → CreateNodeModal
2. service select (service, command) → nodes.create(x, y, service, command) → invoke('create_node', { x, y, service, command })
3. Rust create_node: add_node to graph → spawn(ptys) → se command non-empty: pty.write(id, "{command}\n")
4. PTY reader thread: reads from fd → emitter(data) → app.emit('pty-output-{id}')
     → also stores in Arc<Mutex<Vec<u8>>> buffer
5. Frontend: refreshGraph() → svelte renders TerminalNode → XtermWrapper mounts
6. startTerminalRead: listen('pty-output-{id}') → get_pty_buffer → term.write(buf)

# Tests
cargo test (src-tauri)
3 tests: test_pty_spawn_and_buffer, test_graph_operations, test_group_operations

# Features implementadas
- Terminal nodes com PTY (xterm.js + Rust forkpty) — sempre spawna shell, auto-type comando separado
- Arrastar + redimensionar nodes no canvas (resize handle canto inferior direito)
- Editar label inline (double-click no nome)
- Paleta de cores (12 cores) no NodeHeader
- Service selector modal com ASCII art (shell, opencode, openclaw, codex) — service (metadata) + command (auto-type)
- Groups: container visual (dashed border + header), arrastar move children, renomear, cascade delete
- Sticky notes: markdown files em ~/.local/share/maestri-linux/notes/{id}.md, editor inline
- Group assignment: dropdown ⊞ no header de cada node (assign to group / remove from group)
- Board persistence: auto-save em ~/.local/share/maestri-linux/board.json após cada mutação
- Service vs command: service é metadata (rótulo), command é o que é auto-digitado no PTY
