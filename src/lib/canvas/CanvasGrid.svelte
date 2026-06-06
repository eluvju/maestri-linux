<script lang="ts">
  import { viewport } from './canvasStore';

  const GRID_SIZE = 40;
</script>

<svg width="100%" height="100%" style="position:absolute;top:0;left:0;pointer-events:none;">
  <defs>
    <pattern
      id="grid"
      width={GRID_SIZE}
      height={GRID_SIZE}
      patternUnits="userSpaceOnUse"
      x={$viewport.x % GRID_SIZE}
      y={$viewport.y % GRID_SIZE}
    >
      <path
        d="M {GRID_SIZE} 0 L 0 0 0 {GRID_SIZE}"
        fill="none"
        stroke="#2a2a4a"
        stroke-width="0.5"
      />
    </pattern>
  </defs>
  <rect width="100%" height="100%" fill="url(#grid)" />
</svg>
