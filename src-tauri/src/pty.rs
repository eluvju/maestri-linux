use portable_pty::{CommandBuilder, NativePtySystem, PtyPair, PtySize, PtySystem};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::Mutex;
use uuid::Uuid;

pub struct PtyInstance {
    pub pair: PtyPair,
    pub reader: Box<dyn Read + Send>,
    pub writer: Box<dyn Write + Send>,
}

pub struct PtyManager {
    ptys: HashMap<Uuid, PtyInstance>,
    pty_system: Box<dyn PtySystem + Send>,
}

impl PtyManager {
    pub fn new() -> Self {
        Self {
            ptys: HashMap::new(),
            pty_system: Box::new(NativePtySystem::default()),
        }
    }

    pub fn spawn(&mut self, node_id: Uuid) -> Result<(), String> {
        let pair = self
            .pty_system
            .openpty(PtySize {
                rows: 24,
                cols: 80,
                pixel_width: 640,
                pixel_height: 400,
            })
            .map_err(|e| format!("Failed to open PTY: {}", e))?;

        let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".into());
        let mut cmd = CommandBuilder::new(&shell);
        cmd.env("TERM", "xterm-256color");

        let child = pair
            .slave
            .spawn_command(cmd)
            .map_err(|e| format!("Failed to spawn shell: {}", e))?;

        std::mem::forget(child);

        let reader = pair
            .master
            .try_clone_reader()
            .map_err(|e| format!("Failed to clone reader: {}", e))?;
        let writer = pair
            .master
            .take_writer()
            .map_err(|e| format!("Failed to get writer: {}", e))?;

        self.ptys.insert(
            node_id,
            PtyInstance {
                pair,
                reader,
                writer,
            },
        );

        Ok(())
    }

    pub fn kill(&mut self, node_id: Uuid) {
        self.ptys.remove(&node_id);
    }

    pub fn write(&mut self, node_id: Uuid, data: &[u8]) -> Result<(), String> {
        let inst = self
            .ptys
            .get_mut(&node_id)
            .ok_or_else(|| format!("PTY not found for node {}", node_id))?;
        inst.writer
            .write_all(data)
            .map_err(|e| format!("Failed to write to PTY: {}", e))
    }

    pub fn read(&mut self, node_id: Uuid, buf: &mut [u8]) -> Result<usize, String> {
        let inst = self
            .ptys
            .get_mut(&node_id)
            .ok_or_else(|| format!("PTY not found for node {}", node_id))?;
        inst.reader
            .read(buf)
            .map_err(|e| format!("Failed to read from PTY: {}", e))
    }

    pub fn resize(
        &mut self,
        node_id: Uuid,
        cols: u16,
        rows: u16,
        width: u16,
        height: u16,
    ) -> Result<(), String> {
        let inst = self
            .ptys
            .get_mut(&node_id)
            .ok_or_else(|| format!("PTY not found for node {}", node_id))?;
        inst.pair
            .master
            .resize(PtySize {
                rows,
                cols,
                pixel_width: width,
                pixel_height: height,
            })
            .map_err(|e| format!("Failed to resize PTY: {}", e))
    }

    pub fn has(&self, node_id: Uuid) -> bool {
        self.ptys.contains_key(&node_id)
    }
}

pub struct AppState {
    pub graph: Mutex<crate::state::Graph>,
    pub pty: Mutex<PtyManager>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            graph: Mutex::new(crate::state::Graph::default()),
            pty: Mutex::new(PtyManager::new()),
        }
    }
}
