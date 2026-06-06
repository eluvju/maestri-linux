<script lang="ts">
  let {
    x1, y1, x2, y2,
  }: {
    x1: number;
    y1: number;
    x2: number;
    y2: number;
  } = $props();

  const dx = x2 - x1;
  const dy = y2 - y1;
  const cx1 = x1 + dx * 0.4;
  const cy1 = y1;
  const cx2 = x2 - dx * 0.4;
  const cy2 = y2;
  const d = `M ${x1} ${y1} C ${cx1} ${cy1}, ${cx2} ${cy2}, ${x2} ${y2}`;
</script>

<path {d} fill="none" stroke="#e94560" stroke-width="2" stroke-linecap="round" />
