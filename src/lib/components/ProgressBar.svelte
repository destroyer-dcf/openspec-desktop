<script lang="ts">
  export let completed = 0;
  export let total = 0;
  export let label = "Progreso de tareas";

  $: safeTotal = Math.max(total, 0);
  $: safeCompleted = Math.max(Math.min(completed, safeTotal), 0);
</script>

{#if safeTotal === 0}
  <p class="empty">Sin tareas</p>
{:else}
  <div class="wrap">
    <progress aria-label={label} value={safeCompleted} max={safeTotal}></progress>
    <span>{safeCompleted}/{safeTotal} tareas</span>
  </div>
{/if}

<style>
  .wrap { display: grid; gap: 4px; }
  progress {
    width: 100%;
    height: 10px;
    appearance: none;
    border: none;
    border-radius: 999px;
    overflow: hidden;
  }
  progress::-webkit-progress-bar { background: var(--border-default); }
  progress::-webkit-progress-value { background: var(--accent-color); }
  progress::-moz-progress-bar { background: var(--accent-color); }
  span, .empty { font-size: 12px; color: var(--text-secondary); }
</style>
