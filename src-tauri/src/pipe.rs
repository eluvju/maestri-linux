use std::collections::HashMap;
use uuid::Uuid;

pub struct ActivePipe {
    pub source_id: Uuid,
    pub target_id: Uuid,
}

pub struct PipeEngine {
    pipes: HashMap<Uuid, ActivePipe>,
}

impl PipeEngine {
    pub fn new() -> Self {
        Self {
            pipes: HashMap::new(),
        }
    }

    pub fn add_pipe(&mut self, conn_id: Uuid, source_id: Uuid, target_id: Uuid) {
        self.pipes.insert(
            conn_id,
            ActivePipe {
                source_id,
                target_id,
            },
        );
    }

    pub fn remove_pipe(&mut self, conn_id: Uuid) -> Option<ActivePipe> {
        self.pipes.remove(&conn_id)
    }

    pub fn get_pipes_for_target(&self, target_id: Uuid) -> Vec<&ActivePipe> {
        self.pipes.values().filter(|p| p.target_id == target_id).collect()
    }

    pub fn get_pipes_from_source(&self, source_id: Uuid) -> Vec<&ActivePipe> {
        self.pipes.values().filter(|p| p.source_id == source_id).collect()
    }

    pub fn pump_data(
        &self,
        source_id: Uuid,
        data: &[u8],
        pty_manager: &mut crate::pty::PtyManager,
    ) {
        let pipes: Vec<Uuid> = self
            .pipes
            .values()
            .filter(|p| p.source_id == source_id)
            .map(|p| p.target_id)
            .collect();

        for target_id in pipes {
            let _ = pty_manager.write(target_id, data);
        }
    }
}
