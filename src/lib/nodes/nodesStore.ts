import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface NodeData {
  id: string;
  kind: 'Terminal' | 'StickyNote';
  label: string;
  color: string;
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
      update(n => n.filter(x => x.id !== id));
    },
    async move(id: string, x: number, y: number) {
      await invoke('move_node', { id, x, y });
      update(n => n.map(node => node.id === id ? { ...node, x, y } : node));
    },
    async setLabel(id: string, label: string) {
      await invoke('set_node_label', { id, label });
      update(n => n.map(node => node.id === id ? { ...node, label } : node));
    },
    async setColor(id: string, color: string) {
      await invoke('set_node_color', { id, color });
      update(n => n.map(node => node.id === id ? { ...node, color } : node));
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
