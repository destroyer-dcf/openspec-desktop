<script lang="ts">
  export let percent = 0;

  const size = 144;
  const stroke = 12;
  const radius = (size - stroke) / 2;
  const circumference = 2 * Math.PI * radius;

  $: safePercent = Math.max(0, Math.min(100, Math.round(percent)));
  $: offset = circumference * (1 - safePercent / 100);
</script>

<div class="circle-wrap" aria-label="Progreso global" role="img">
  <svg width={size} height={size} viewBox={`0 0 ${size} ${size}`}>
    <circle class="track" cx={size / 2} cy={size / 2} r={radius}></circle>
    <circle class="value" cx={size / 2} cy={size / 2} r={radius} stroke-dasharray={circumference} stroke-dashoffset={offset}></circle>
  </svg>
  <span>{safePercent}%</span>
</div>

<style>
  .circle-wrap { position: relative; width: 144px; height: 144px; display: grid; place-items: center; }
  svg { transform: rotate(-90deg); }
  circle { fill: none; stroke-width: 12; }
  .track { stroke: var(--border-default); }
  .value { stroke: var(--accent-color); stroke-linecap: round; transition: stroke-dashoffset 0.2s ease; }
  span { position: absolute; font-size: 24px; font-weight: 700; color: var(--text-primary); }
</style>
