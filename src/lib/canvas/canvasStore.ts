import { writable } from 'svelte/store';

export interface Viewport {
  x: number;
  y: number;
  zoom: number;
}

function createViewportStore() {
  const { subscribe, set, update } = writable<Viewport>({ x: 0, y: 0, zoom: 1 });

  return {
    subscribe,
    pan(dx: number, dy: number) {
      update((v) => ({ ...v, x: v.x + dx, y: v.y + dy }));
    },
    zoomAt(amount: number, cx: number, cy: number) {
      update((v) => {
        const newZoom = Math.max(0.1, Math.min(5, v.zoom * (1 + amount)));
        const scale = newZoom / v.zoom;
        return {
          x: cx - scale * (cx - v.x),
          y: cy - scale * (cy - v.y),
          zoom: newZoom,
        };
      });
    },
    setViewport(v: Viewport) {
      set(v);
    },
    screenToCanvas(v: Viewport, sx: number, sy: number): { x: number; y: number } {
      return {
        x: (sx - v.x) / v.zoom,
        y: (sy - v.y) / v.zoom,
      };
    },
  };
}

export const viewport = createViewportStore();
