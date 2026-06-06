use portable_pty::{CommandBuilder, NativePtySystem, PtyPair, PtySize, PtySystem};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

const BUFFER_CAPACITY: usize = 64 * 1024;

type PtyEmitter = Box<dyn Fn(&[u8]) + Send>;

pub struct PtyInstance {
    pub pair: PtyPair,
    pub reader: Box<dyn std::io::Read + Send>,
    pub writer: Box<dyn std::io::Write + Send>,
    pub buffer: Arc<Mutex<Vec<u8>>>,
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

    pub fn spawn(&mut self, node_id: Uuid, emitter: PtyEmitter) -> Result<(), String> {
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

        let buffer = Arc::new(Mutex::new(Vec::<u8>::new()));
        let buffer_clone = buffer.clone();

        self.ptys.insert(
            node_id,
            PtyInstance {
                pair,
                reader: Box::new(std::io::empty()),
                writer,
                buffer,
            },
        );

        std::thread::spawn(move || {
            let mut buf = vec![0u8; 4096];
            let mut reader_ref = reader;
            loop {
                match reader_ref.read(&mut buf) {
                    Ok(n) if n > 0 => {
                        let data: Vec<u8> = buf[..n].to_vec();
                        emitter(&data);
                        let mut b = buffer_clone.lock().unwrap();
                        if b.len() + data.len() > BUFFER_CAPACITY {
                            let excess = b.len() + data.len() - BUFFER_CAPACITY;
                            b.drain(..excess);
                        }
                        b.extend_from_slice(&data);
                    }
                    Ok(_) => break,
                    Err(_) => break,
                }
            }
        });

        Ok(())
    }

    pub fn get_buffer(&self, node_id: Uuid) -> Vec<u8> {
        if let Some(inst) = self.ptys.get(&node_id) {
            if let Ok(b) = inst.buffer.lock() {
                return b.clone();
            }
        }
        Vec::new()
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

    pub fn resize(&mut self, node_id: Uuid, cols: u16, rows: u16, width: u16, height: u16) -> Result<(), String> {
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
