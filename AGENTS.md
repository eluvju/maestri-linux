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
- src-tauri/src/lib.rs: create_node (emitter + buffer), log_msg
- src-tauri/src/pty.rs: PtyManager::spawn (reader thread → emitter + buffer)
- src/lib/terminal/terminal.ts: startTerminalRead (listen + buffer replay)
- src/lib/terminal/XtermWrapper.svelte: xterm.js init + listener registration

# Event flow
1. double-click → App.svelte addNode → nodes.create → invoke('create_node')
2. Rust create_node: add_node to graph → spawn(ptys, emitter) → spawn returns
3. PTY reader thread: reads from fd → emitter(data) → app.emit('pty-output-{id}')
     → also stores in Arc<Mutex<Vec<u8>>> buffer
4. Frontend: refreshGraph() → svelte renders TerminalNode → XtermWrapper mounts
5. startTerminalRead: listen('pty-output-{id}') → get_pty_buffer → term.write(buf)

# Tests
cargo test (src-tauri)
2 tests: test_pty_spawn_and_buffer, test_graph_operations
