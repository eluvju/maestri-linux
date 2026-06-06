import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface NodeData {
  id: string;
  kind: 'Terminal' | 'StickyNote';
  label: string;
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface ConnectionData {
  id: string;
  source_id: string;
  source_fd: 'Stdout' | 'Stderr';
  target_id: string;
  target_fd: 'Stdin';
}

function createNodesStore() {
  const { subscribe, set, update } = writable<NodeData[]>([]);

  return {
    subscribe,
    async create(x: number, y: number): Promise<string> {
      const id = await invoke<string>('create_node', { x, y });
      return id;
    },
    async remove(id: string) {
      await invoke('remove_node', { id });
    },
    async move(id: string, x: number, y: number) {
      await invoke('move_node', { id, x, y });
    },
    refresh(nodes: NodeData[]) {
      set(nodes);
    },
  };
}

function createConnectionsStore() {
  const { subscribe, set, update } = writable<ConnectionData[]>([]);

  return {
    subscribe,
    async connect(sourceId: string, targetId: string): Promise<string> {
      return await invoke<string>('connect_nodes', { sourceId, targetId });
    },
    async disconnect(id: string) {
      await invoke('disconnect_nodes', { id });
    },
    refresh(connections: ConnectionData[]) {
      set(connections);
    },
  };
}

export const nodes = createNodesStore();
export const connections = createConnectionsStore();
