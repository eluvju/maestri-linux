mod pipe;
mod pty;
mod state;

use pty::AppState;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use tauri::{AppHandle, Emitter, State};
use uuid::Uuid;

fn log_to_file(msg: &str) {
    if let Ok(mut f) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/tmp/maestri-linux.log")
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        let secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        let _ = writeln!(f, "[{}] {}", secs, msg);
    }
}

fn save_and_log(graph: &state::Graph) {
    graph.save();
    log_to_file("[save] board saved");
}

#[tauri::command]
fn log_msg(level: String, msg: String) {
    log_to_file(&format!("[{}] {}", level, msg));
}

#[tauri::command]
fn create_node(app_state: State<AppState>, x: f64, y: f64, service: String, command: String, app: AppHandle) -> Result<String, String> {
    let mut graph = app_state.graph.lock().unwrap();
    let label = if service == "shell" { String::new() } else { service.clone() };
    let id = graph.add_node_label(x, y, service.clone(), label);
    let id_str = id.to_string();
    log_to_file(&format!("[create_node] id={} x={} y={} service={} command={}", id_str, x, y, service, command));
    let app_clone = app.clone();
    let node_id = id;
    let id_for_log = id_str.clone();
    let emitter: Box<dyn Fn(&[u8]) + Send> = Box::new(move |data| {
        let _ = app_clone.emit(&format!("pty-output-{}", node_id), data.to_vec());
        log_to_file(&format!("[emitter] node={} emitted {} bytes", id_for_log, data.len()));
    });
    drop(graph);
    app_state.pty.lock().unwrap().spawn(id, emitter, "shell").map_err(|e| {
        log_to_file(&format!("[create_node] spawn failed: {}", e));
        e
    })?;
    log_to_file(&format!("[create_node] spawn OK id={}", id_str));
    if !command.is_empty() {
        let cmd = format!("{}\n", command);
        let _ = app_state.pty.lock().unwrap().write(id, cmd.as_bytes());
        log_to_file(&format!("[create_node] auto-typed: {}", cmd.trim()));
    }
    app_state.graph.lock().unwrap().save();
    Ok(id_str)
}

#[tauri::command]
fn remove_node(app_state: State<AppState>, id: String) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    app_state.pty.lock().unwrap().kill(uuid);
    let mut graph = app_state.graph.lock().unwrap();
    graph.remove_node(uuid);
    graph.save();
    Ok(())
}

#[tauri::command]
fn connect_nodes(app_state: State<AppState>, source_id: String, target_id: String) -> Result<String, String> {
    let src = Uuid::parse_str(&source_id).map_err(|e| e.to_string())?;
    let tgt = Uuid::parse_str(&target_id).map_err(|e| e.to_string())?;
    let mut graph = app_state.graph.lock().unwrap();
    let conn_id = graph.add_connection(src, tgt)?;
    graph.save();
    Ok(conn_id.to_string())
}

#[tauri::command]
fn disconnect_nodes(app_state: State<AppState>, id: String) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let mut graph = app_state.graph.lock().unwrap();
    graph.remove_connection(uuid);
    graph.save();
    Ok(())
}

#[tauri::command]
fn get_graph(app_state: State<AppState>) -> Result<state::Graph, String> {
    Ok(app_state.graph.lock().unwrap().clone())
}

#[tauri::command]
fn move_node(app_state: State<AppState>, id: String, x: f64, y: f64) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let mut graph = app_state.graph.lock().unwrap();
    if let Some(node) = graph.get_node_mut(uuid) {
        node.x = x;
        node.y = y;
        graph.save();
        Ok(())
    } else {
        Err("Node not found".into())
    }
}

#[tauri::command]
fn set_node_label(app_state: State<AppState>, id: String, label: String) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let mut graph = app_state.graph.lock().unwrap();
    if let Some(node) = graph.get_node_mut(uuid) {
        node.label = label;
        graph.save();
        Ok(())
    } else {
        Err("Node not found".into())
    }
}

#[tauri::command]
fn set_node_color(app_state: State<AppState>, id: String, color: String) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let mut graph = app_state.graph.lock().unwrap();
    if let Some(node) = graph.get_node_mut(uuid) {
        node.color = color;
        graph.save();
        Ok(())
    } else {
        Err("Node not found".into())
    }
}

#[tauri::command]
fn create_group(app_state: State<AppState>, label: String) -> Result<String, String> {
    let mut graph = app_state.graph.lock().unwrap();
    let id = graph.add_group(label);
    let id_str = id.to_string();
    graph.save();
    Ok(id_str)
}

#[tauri::command]
fn remove_group(app_state: State<AppState>, id: String) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let mut graph = app_state.graph.lock().unwrap();
    graph.remove_group(uuid);
    graph.save();
    Ok(())
}

#[tauri::command]
fn add_node_to_group(app_state: State<AppState>, node_id: String, group_id: String) -> Result<(), String> {
    let nid = Uuid::parse_str(&node_id).map_err(|e| e.to_string())?;
    let gid = Uuid::parse_str(&group_id).map_err(|e| e.to_string())?;
    let mut graph = app_state.graph.lock().unwrap();
    graph.add_node_to_group(nid, gid)?;
    graph.save();
    Ok(())
}

#[tauri::command]
fn remove_node_from_group(app_state: State<AppState>, node_id: String) -> Result<(), String> {
    let nid = Uuid::parse_str(&node_id).map_err(|e| e.to_string())?;
    let mut graph = app_state.graph.lock().unwrap();
    graph.remove_node_from_group(nid);
    graph.save();
    Ok(())
}

#[tauri::command]
fn move_group(app_state: State<AppState>, id: String, dx: f64, dy: f64) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let mut graph = app_state.graph.lock().unwrap();
    graph.move_group(uuid, dx, dy)?;
    graph.save();
    Ok(())
}

#[tauri::command]
fn set_group_label(app_state: State<AppState>, id: String, label: String) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let mut graph = app_state.graph.lock().unwrap();
    if let Some(group) = graph.get_group_mut(uuid) {
        group.label = label;
        graph.save();
        Ok(())
    } else {
        Err("Group not found".into())
    }
}

#[tauri::command]
fn resize_pty(app_state: State<AppState>, id: String, cols: u16, rows: u16, width: u16, height: u16) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    app_state.pty.lock().unwrap().resize(uuid, cols, rows, width, height)
}

#[tauri::command]
fn get_pty_buffer(app_state: State<AppState>, id: String) -> Result<Vec<u8>, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    Ok(app_state.pty.lock().unwrap().get_buffer(uuid))
}

#[tauri::command]
fn set_node_size(app_state: State<AppState>, id: String, width: f64, height: f64) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let mut graph = app_state.graph.lock().unwrap();
    graph.set_node_size(uuid, width, height)?;
    graph.save();
    Ok(())
}

#[tauri::command]
fn create_sticky_note(app_state: State<AppState>, x: f64, y: f64, content: String) -> Result<String, String> {
    let mut graph = app_state.graph.lock().unwrap();
    let id = graph.add_sticky_note(x, y, &content);
    let id_str = id.to_string();
    graph.save();
    log_to_file(&format!("[create_note] id={} x={} y={}", id_str, x, y));
    Ok(id_str)
}

#[tauri::command]
fn get_note_content(id: String) -> Result<String, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let path = state::notes_dir().join(format!("{}.md", uuid));
    fs::read_to_string(&path).map_err(|e| format!("Failed to read note: {}", e))
}

#[tauri::command]
fn set_note_content(id: String, content: String) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let path = state::notes_dir().join(format!("{}.md", uuid));
    fs::write(&path, &content).map_err(|e| format!("Failed to write note: {}", e))
}

#[tauri::command]
fn write_pty(app_state: State<AppState>, id: String, data: Vec<u8>) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    app_state.pty.lock().unwrap().write(uuid, &data)
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
             create_node,
            remove_node,
            connect_nodes,
            disconnect_nodes,
            get_graph,
            move_node,
            set_node_label,
            set_node_color,
            create_group,
            remove_group,
            add_node_to_group,
            remove_node_from_group,
            move_group,
            set_group_label,
            resize_pty,
            write_pty,
            get_pty_buffer,
            log_msg,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use crate::pty::PtyManager;
    use crate::state::Graph;
    use std::sync::{Arc, Mutex};
    use uuid::Uuid;

    #[test]
    fn test_pty_spawn_and_buffer() {
        let node_id = Uuid::new_v4();
        let buf_store = Arc::new(Mutex::new(Vec::<u8>::new()));
        let buf_clone = buf_store.clone();

        let emitter: Box<dyn Fn(&[u8]) + Send> = Box::new(move |data| {
            buf_clone.lock().unwrap().extend_from_slice(data);
        });

        let mut pty = PtyManager::new();
        pty.spawn(node_id, emitter, "shell").expect("spawn should succeed");

        std::thread::sleep(std::time::Duration::from_millis(800));

        let buf = pty.get_buffer(node_id);
        assert!(!buf.is_empty(), "PTY should have produced output, got {} bytes", buf.len());
        eprintln!("PTY output: {} bytes: {:?}", buf.len(), String::from_utf8_lossy(&buf));

        let emitted = buf_store.lock().unwrap().clone();
        assert!(!emitted.is_empty(), "Emitter should have received data, got {} bytes", emitted.len());
        eprintln!("Emitted: {} bytes: {:?}", emitted.len(), String::from_utf8_lossy(&emitted));

        pty.write(node_id, b"echo PTY_WORKS\n").expect("write should succeed");
        std::thread::sleep(std::time::Duration::from_millis(500));

        let buf2 = pty.get_buffer(node_id);
        let s2 = String::from_utf8_lossy(&buf2);
        assert!(buf2.len() > buf.len() || s2.contains("PTY_WORKS"),
            "PTY should have more output after writing a command. Was {} now {}", buf.len(), buf2.len());
        eprintln!("After write: {} bytes: {:?}", buf2.len(), s2);

        pty.kill(node_id);
        assert!(!pty.has(node_id), "PTY should be removed");
    }

    #[test]
    fn test_graph_operations() {
        let mut graph = Graph::default();
        let id1 = graph.add_node(100.0, 200.0, "shell".into());
        let id2 = graph.add_node(300.0, 400.0, "opencode".into());

        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.nodes[1].service, "opencode");

        let n = graph.get_node_mut(id1).unwrap();
        n.x = 150.0;
        n.y = 250.0;
        assert_eq!(n.x, 150.0);

        let conn_id = graph.add_connection(id1, id2).expect("connect should succeed");
        assert_eq!(graph.connections.len(), 1);
        graph.remove_connection(conn_id);
        assert_eq!(graph.connections.len(), 0);

        graph.remove_node(id1);
        assert_eq!(graph.nodes.len(), 1);
        assert!(graph.get_node(id2).is_some());
    }

    #[test]
    fn test_group_operations() {
        let mut graph = Graph::default();
        let gid = graph.add_group("My Project".into());
        let nid = graph.add_node(100.0, 200.0, "shell".into());

        graph.add_node_to_group(nid, gid).expect("add to group");
        assert_eq!(graph.nodes.iter().find(|n| n.id == nid).unwrap().group_id, Some(gid));
        assert_eq!(graph.groups[0].child_ids.len(), 1);

        graph.remove_node_from_group(nid);
        assert!(graph.nodes.iter().find(|n| n.id == nid).unwrap().group_id.is_none());
        assert!(graph.groups[0].child_ids.is_empty());

        graph.remove_group(gid);
        assert_eq!(graph.groups.len(), 0);
    }
}
