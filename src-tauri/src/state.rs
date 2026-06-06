use serde::{Deserialize, Serialize};
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
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Node {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            kind: NodeKind::Terminal,
            label: String::new(),
            x,
            y,
            width: 640.0,
            height: 400.0,
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
pub struct Graph {
    pub nodes: Vec<Node>,
    pub connections: Vec<Connection>,
    pub viewport: Viewport,
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            connections: Vec::new(),
            viewport: Viewport::default(),
        }
    }
}

impl Graph {
    pub fn add_node(&mut self, x: f64, y: f64) -> Uuid {
        let node = Node::new(x, y);
        let id = node.id;
        self.nodes.push(node);
        id
    }

    pub fn remove_node(&mut self, id: Uuid) -> Option<Node> {
        self.connections.retain(|c| c.source_id != id && c.target_id != id);
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
}
