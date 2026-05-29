<script lang="ts">
  import { ArrowLeft, FileText, Pencil } from "lucide-svelte";
  import type { UiKey } from "$lib/i18n";
  import type { Artifact, Change } from "$lib/types";

  export let change: Change;
  export let t: (key: UiKey) => string;
  export let onOpenArtifact: (artifact: Artifact) => void;
  export let onBack: () => void;
</script>

<section>
  <div class="top">
    <button on:click={onBack} aria-label={t("back")} title={t("back")}><ArrowLeft size={14} /> {t("back")}</button>
    <h2>{change.name}</h2>
  </div>

  <ul>
    {#each change.artifacts as artifact}
      <li>
        <div class="row">
          <span>{artifact.name} · {artifact.present ? t("present") : t("pending")}</span>
          <div class="actions">
            <button class="primary" aria-label={t("open")} title={t("open")} on:click={() => onOpenArtifact(artifact)}><FileText size={14} /></button>
            <button class="primary" aria-label={t("edit")} title={t("edit")} on:click={() => onOpenArtifact(artifact)}><Pencil size={14} /></button>
          </div>
        </div>
      </li>
    {/each}
  </ul>
</section>

<style>
  section { min-width: 0; }
  .top { display: flex; justify-content: space-between; align-items: center; gap: 8px; }
  h2 { margin: 0; font-size: var(--font-size-base); }
  ul { list-style: none; padding: 0; display: grid; gap: 8px; }
  .row { display: flex; justify-content: space-between; align-items: center; gap: 10px; border: 1px solid var(--border-default); padding: 8px; border-radius: 8px; background: var(--bg-secondary); color: var(--text-primary); font-size: var(--font-size-small); min-width: 0; }
  .actions { display: flex; gap: 6px; }
</style>
