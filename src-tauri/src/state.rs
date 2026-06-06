use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Viewport {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            zoom: 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum NodeKind {
    Terminal,
    StickyNote,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum FdKind {
    Stdout,
    Stderr,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum FdTarget {
    Stdin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: Uuid,
    pub kind: NodeKind,
    pub label: String,
    pub color: String,
    pub service: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub group_id: Option<Uuid>,
}

impl Node {
    pub fn new(x: f64, y: f64, service: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            kind: NodeKind::Terminal,
            label: String::new(),
            color: "#0f3460".into(),
            service,
            x,
            y,
            width: 640.0,
            height: 400.0,
            group_id: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub id: Uuid,
    pub source_id: Uuid,
    pub source_fd: FdKind,
    pub target_id: Uuid,
    pub target_fd: FdTarget,
}

impl Connection {
    pub fn new(source_id: Uuid, target_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            source_id,
            source_fd: FdKind::Stdout,
            target_id,
            target_fd: FdTarget::Stdin,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: Uuid,
    pub label: String,
    pub color: String,
    pub child_ids: Vec<Uuid>,
}

impl Group {
    pub fn new(label: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            label,
            color: "#7c4dff".into(),
            child_ids: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub connections: Vec<Connection>,
    pub groups: Vec<Group>,
    pub viewport: Viewport,
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            connections: Vec::new(),
            groups: Vec::new(),
            viewport: Viewport::default(),
        }
    }
}

fn data_dir() -> PathBuf {
    let base = dirs::data_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
    let dir = base.join("maestri-linux");
    let _ = fs::create_dir_all(&dir);
    dir
}

pub fn notes_dir() -> PathBuf {
    let dir = data_dir().join("notes");
    let _ = fs::create_dir_all(&dir);
    dir
}

fn save_path() -> PathBuf {
    data_dir().join("board.json")
}

impl Graph {
    pub fn add_node(&mut self, x: f64, y: f64, service: String) -> Uuid {
        let node = Node::new(x, y, service);
        let id = node.id;
        self.nodes.push(node);
        id
    }

    pub fn add_node_label(&mut self, x: f64, y: f64, service: String, label: String) -> Uuid {
        let mut node = Node::new(x, y, service);
        node.label = label;
        let id = node.id;
        self.nodes.push(node);
        id
    }

    pub fn remove_node(&mut self, id: Uuid) -> Option<Node> {
        self.connections.retain(|c| c.source_id != id && c.target_id != id);
        for g in &mut self.groups {
            g.child_ids.retain(|c| *c != id);
        }
        let idx = self.nodes.iter().position(|n| n.id == id)?;
        Some(self.nodes.remove(idx))
    }

    pub fn add_connection(&mut self, source_id: Uuid, target_id: Uuid) -> Result<Uuid, String> {
        if source_id == target_id {
            return Err("Cannot connect node to itself".into());
        }
        if !self.nodes.iter().any(|n| n.id == source_id) {
            return Err("Source node not found".into());
        }
        if !self.nodes.iter().any(|n| n.id == target_id) {
            return Err("Target node not found".into());
        }
        if self.connections.iter().any(|c| c.source_id == source_id && c.target_id == target_id) {
            return Err("Connection already exists".into());
        }
        let conn = Connection::new(source_id, target_id);
        let id = conn.id;
        self.connections.push(conn);
        Ok(id)
    }

    pub fn remove_connection(&mut self, id: Uuid) -> Option<Connection> {
        let idx = self.connections.iter().position(|c| c.id == id)?;
        Some(self.connections.remove(idx))
    }

    pub fn get_node(&self, id: Uuid) -> Option<&Node> {
        self.nodes.iter().find(|n| n.id == id)
    }

    pub fn get_node_mut(&mut self, id: Uuid) -> Option<&mut Node> {
        self.nodes.iter_mut().find(|n| n.id == id)
    }

    pub fn add_group(&mut self, label: String) -> Uuid {
        let group = Group::new(label);
        let id = group.id;
        self.groups.push(group);
        id
    }

    pub fn remove_group(&mut self, id: Uuid) -> Option<Group> {
        for n in &mut self.nodes {
            if n.group_id == Some(id) {
                n.group_id = None;
            }
        }
        let idx = self.groups.iter().position(|g| g.id == id)?;
        Some(self.groups.remove(idx))
    }

    pub fn get_group_mut(&mut self, id: Uuid) -> Option<&mut Group> {
        self.groups.iter_mut().find(|g| g.id == id)
    }

    pub fn add_node_to_group(&mut self, node_id: Uuid, group_id: Uuid) -> Result<(), String> {
        if !self.nodes.iter().any(|n| n.id == node_id) {
            return Err("Node not found".into());
        }
        if !self.groups.iter().any(|g| g.id == group_id) {
            return Err("Group not found".into());
        }
        if let Some(node) = self.get_node_mut(node_id) {
            node.group_id = Some(group_id);
        }
        if let Some(group) = self.get_group_mut(group_id) {
            if !group.child_ids.contains(&node_id) {
                group.child_ids.push(node_id);
            }
        }
        Ok(())
    }

    pub fn remove_node_from_group(&mut self, node_id: Uuid) {
        if let Some(node) = self.get_node_mut(node_id) {
            node.group_id = None;
        }
        for g in &mut self.groups {
            g.child_ids.retain(|c| *c != node_id);
        }
    }

    pub fn set_node_size(&mut self, id: Uuid, width: f64, height: f64) -> Result<(), String> {
        if let Some(node) = self.get_node_mut(id) {
            node.width = width;
            node.height = height;
            Ok(())
        } else {
            Err("Node not found".into())
        }
    }

    pub fn move_group(&mut self, group_id: Uuid, dx: f64, dy: f64) -> Result<(), String> {
        let ids = self.groups.iter().find(|g| g.id == group_id).ok_or("Group not found")?.child_ids.clone();
        for id in &ids {
            if let Some(node) = self.get_node_mut(*id) {
                node.x += dx;
                node.y += dy;
            }
        }
        Ok(())
    }

    pub fn add_sticky_note(&mut self, x: f64, y: f64, content: &str) -> Uuid {
        let mut node = Node::new(x, y, "note".into());
        node.kind = NodeKind::StickyNote;
        node.label = String::new();
        let id = node.id;
        let note_path = notes_dir().join(format!("{}.md", id));
        let _ = fs::write(&note_path, content);
        self.nodes.push(node);
        id
    }

    pub fn save(&self) {
        let path = save_path();
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(&path, json);
        }
    }

    pub fn load_or_default() -> Self {
        let path = save_path();
        if let Ok(data) = fs::read_to_string(&path) {
            if let Ok(graph) = serde_json::from_str(&data) {
                return graph;
            }
        }
        Graph::default()
    }
}
