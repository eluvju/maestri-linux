import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface NodeData {
  id: string;
  kind: 'Terminal' | 'StickyNote';
  label: string;
  color: string;
  service: string;
  x: number;
  y: number;
  width: number;
  height: number;
  group_id: string | null;
}

export interface ConnectionData {
  id: string;
  source_id: string;
  source_fd: 'Stdout' | 'Stderr';
  target_id: string;
  target_fd: 'Stdin';
}

export interface GroupData {
  id: string;
  label: string;
  color: string;
  child_ids: string[];
}

function createNodesStore() {
  const { subscribe, set, update } = writable<NodeData[]>([]);

  return {
    subscribe,
    async create(x: number, y: number, service = 'shell', command = ''): Promise<string> {
      const id = await invoke<string>('create_node', { x, y, service, command });
      return id;
    },

    async createNote(x: number, y: number, content = ''): Promise<string> {
      const id = await invoke<string>('create_sticky_note', { x, y, content });
      return id;
    },

    async getNoteContent(id: string): Promise<string> {
      return await invoke<string>('get_note_content', { id });
    },

    async setNoteContent(id: string, content: string) {
      await invoke('set_note_content', { id, content });
    },

    async setSize(id: string, width: number, height: number) {
      await invoke('set_node_size', { id, width, height });
      update(n => n.map(node => node.id === id ? { ...node, width, height } : node));
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

function createGroupsStore() {
  const { subscribe, set, update } = writable<GroupData[]>([]);

  return {
    subscribe,
    async create(label = 'New Group'): Promise<string> {
      const id = await invoke<string>('create_group', { label });
      return id;
    },
    async remove(id: string) {
      await invoke('remove_group', { id });
      update(g => g.filter(x => x.id !== id));
    },
    async addNode(nodeId: string, groupId: string) {
      await invoke('add_node_to_group', { nodeId, groupId });
    },
    async removeNode(nodeId: string) {
      await invoke('remove_node_from_group', { nodeId });
    },
    async move(id: string, dx: number, dy: number) {
      await invoke('move_group', { id, dx, dy });
    },
    async setLabel(id: string, label: string) {
      await invoke('set_group_label', { id, label });
      update(g => g.map(x => x.id === id ? { ...x, label } : x));
    },
    refresh(groups: GroupData[]) {
      set(groups);
    },
  };
}

export const nodes = createNodesStore();
export const connections = createConnectionsStore();
export const groups = createGroupsStore();

export async function assignToGroup(nodeId: string, groupId: string | null) {
  if (groupId) {
    await invoke('add_node_to_group', { nodeId, groupId });
  } else {
    await invoke('remove_node_from_group', { nodeId });
  }
}
