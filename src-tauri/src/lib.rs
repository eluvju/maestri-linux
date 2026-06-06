mod pipe;
mod pty;
mod state;

use pty::AppState;
use tauri::{AppHandle, State};
use uuid::Uuid;

#[tauri::command]
fn create_node(app_state: State<AppState>, x: f64, y: f64, app: AppHandle) -> Result<String, String> {
    let id = app_state.graph.lock().unwrap().add_node(x, y);
    let id_str = id.to_string();
    app_state.pty.lock().unwrap().spawn(id, app).map_err(|e| e)?;
    Ok(id_str)
}

#[tauri::command]
fn remove_node(app_state: State<AppState>, id: String) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    app_state.pty.lock().unwrap().kill(uuid);
    app_state.graph.lock().unwrap().remove_node(uuid);
    Ok(())
}

#[tauri::command]
fn connect_nodes(app_state: State<AppState>, source_id: String, target_id: String) -> Result<String, String> {
    let src = Uuid::parse_str(&source_id).map_err(|e| e.to_string())?;
    let tgt = Uuid::parse_str(&target_id).map_err(|e| e.to_string())?;
    let conn_id = app_state.graph.lock().unwrap().add_connection(src, tgt)?;
    Ok(conn_id.to_string())
}

#[tauri::command]
fn disconnect_nodes(app_state: State<AppState>, id: String) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    app_state.graph.lock().unwrap().remove_connection(uuid);
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
        Ok(())
    } else {
        Err("Node not found".into())
    }
}

#[tauri::command]
fn resize_pty(app_state: State<AppState>, id: String, cols: u16, rows: u16, width: u16, height: u16) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    app_state.pty.lock().unwrap().resize(uuid, cols, rows, width, height)
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
            resize_pty,
            write_pty,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
