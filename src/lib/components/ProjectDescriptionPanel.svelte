<script lang="ts">
  import { Info } from "lucide-svelte";

  export let contexto: string | null | undefined;
  export let percent = 0;
  export let completedTasks = 0;
  export let totalTasks = 0;
  export let activeChanges = 0;
  export let archivedChanges = 0;

  type ContextoPair = { label: string; value: string };

  function parseContexto(raw: string | null | undefined): ContextoPair[] {
    if (!raw || raw.trim() === "") return [];
    return raw
      .split("\n")
      .map((line) => line.trim())
      .filter((line) => line.length > 0)
      .map((line) => {
        const colonIdx = line.indexOf(":");
        if (colonIdx === -1) return { label: line, value: "No definido" };
        const label = line.slice(0, colonIdx).trim();
        const value = line.slice(colonIdx + 1).trim() || "No definido";
        return { label, value };
      });
  }

  function clamp(value: number) {
    if (!Number.isFinite(value)) return 0;
    return Math.max(0, Math.min(100, value));
  }

  $: pairs = parseContexto(contexto);
  $: safePairs = pairs.length > 0 ? pairs : [{ label: "Contexto", value: "No definido" }];
  $: totalChanges = activeChanges + archivedChanges;
  $: taskPercent = clamp(percent);
  $: activePercent = totalChanges === 0 ? 0 : clamp((activeChanges / totalChanges) * 100);
</script>

<section class="project-panel" aria-label="Descripción del proyecto">
  <div class="panel-head">
    <div class="left">
      <h2 class="panel-title"><Info size={15} /> Proyecto</h2>
      <ul class="pairs">
        {#each safePairs as pair}
          <li class="pair">
            <span class="label">{pair.label}</span>
            <span class="value" title={pair.value}>{pair.value}</span>
          </li>
        {/each}
      </ul>
    </div>

    <div class="right" aria-label="Estado general del proyecto">
      <div class="summary-head">
        <strong>{Math.round(taskPercent)}%</strong>
        <span>Estado general</span>
      </div>
      <div class="metric">
        <div class="metric-row">
          <span>Tareas</span>
          <span>{completedTasks}/{totalTasks}</span>
        </div>
        <div class="bar"><span style={`width:${taskPercent}%`}></span></div>
      </div>
      <div class="metric">
        <div class="metric-row">
          <span>Cambios activos</span>
          <span>{activeChanges}/{totalChanges}</span>
        </div>
        <div class="bar"><span style={`width:${activePercent}%`}></span></div>
      </div>
      <div class="stats">
        <span>Total cambios: {totalChanges}</span>
        <span>Archivados: {archivedChanges}</span>
      </div>
    </div>
  </div>
</section>

<style>
  .project-panel {
    border: 1px solid var(--border-default);
    border-radius: 8px;
    padding: 8px 12px;
    background: var(--bg-secondary);
    color: var(--text-primary);
    margin-bottom: 10px;
  }
  .panel-title {
    display: flex;
    align-items: center;
    gap: 6px;
    margin: 0 0 6px 0;
    font-size: var(--font-size-base);
  }
  .panel-head {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: center;
    gap: 12px;
    min-width: 0;
  }
  .left {
    min-width: 0;
  }
  .right {
    min-width: 280px;
    display: grid;
    gap: 8px;
    font-size: var(--font-size-small);
  }
  .summary-head {
    display: flex;
    align-items: baseline;
    gap: 8px;
  }
  .summary-head strong {
    font-size: calc(var(--font-size-base) + 8px);
    line-height: 1;
  }
  .summary-head span {
    color: var(--text-secondary);
  }
  .metric {
    display: grid;
    gap: 4px;
  }
  .metric-row {
    display: flex;
    justify-content: space-between;
    gap: 8px;
    color: var(--text-secondary);
  }
  .bar {
    height: 8px;
    border-radius: 999px;
    background: var(--border-default);
    overflow: hidden;
  }
  .bar span {
    display: block;
    height: 100%;
    border-radius: 999px;
    background: var(--accent-color);
  }
  .stats {
    display: flex;
    gap: 10px;
    flex-wrap: wrap;
    color: var(--text-secondary);
  }
  .pairs {
    list-style: none;
    margin: 0;
    padding: 0;
    display: grid;
    gap: 4px;
  }
  .pair {
    display: flex;
    align-items: baseline;
    gap: 4px;
    min-width: 0;
  }
  .label {
    font-size: var(--font-size-small);
    color: var(--text-secondary);
    white-space: nowrap;
    flex-shrink: 0;
  }
  .label::after {
    content: ":";
  }
  .value {
    font-size: var(--font-size-small);
    color: var(--text-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 100%;
  }
  @media (max-width: 1180px) {
    .panel-head {
      grid-template-columns: 1fr;
      align-items: start;
    }
    .right {
      min-width: 0;
    }
  }
</style>
