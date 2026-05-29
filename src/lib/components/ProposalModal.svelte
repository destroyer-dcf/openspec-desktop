<script lang="ts">
  import type { ProposalType } from "$lib/types";

  export let open = false;
  export let title = "Propuesta";
  export let name = "";
  export let proposalType: ProposalType = "feature";
  export let content = "";
  export let saving = false;
  export let error = "";
  export let onClose: () => void;
  export let onSave: (payload: { name: string; proposalType: ProposalType; content: string }) => void;

  let localName = "";
  let localType: ProposalType = "feature";
  let localContent = "";

  $: if (open) {
    localName = name;
    localType = proposalType;
    localContent = content;
  }
</script>

{#if open}
  <div class="modal" role="dialog" aria-modal="true" aria-label={title}>
    <div class="sheet">
      <h3>{title}</h3>

      <label>
        Nombre
        <input bind:value={localName} placeholder="nombre de propuesta" />
      </label>

      <label>
        Tipo
        <select bind:value={localType}>
          <option value="feature">Feature</option>
          <option value="bug">Bug</option>
        </select>
      </label>

      <label>
        Propuesta (Markdown)
        <textarea bind:value={localContent} placeholder="# Resumen\n\nDescribe la propuesta..."></textarea>
      </label>

      {#if error}
        <p class="error">{error}</p>
      {/if}

      <div class="actions">
        <button on:click={onClose} disabled={saving}>Descartar</button>
        <button
          class="primary"
          on:click={() => onSave({ name: localName, proposalType: localType, content: localContent })}
          disabled={saving || !localName.trim()}
        >
          {saving ? "Guardando..." : "Guardar"}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal { position: fixed; inset: 0; background: var(--overlay-backdrop); display: grid; place-items: center; z-index: 60; }
  .sheet { width: min(780px, calc(100vw - 24px)); max-height: calc(100vh - 24px); overflow: auto; border: 1px solid var(--border-default); border-radius: 10px; background: var(--bg-primary); padding: 12px; display: grid; gap: 10px; }
  h3 { margin: 0; }
  label { display: grid; gap: 6px; font-size: var(--font-size-small); color: var(--text-secondary); }
  textarea { min-height: 260px; font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, Liberation Mono, Courier New, monospace; }
  .actions { display: flex; justify-content: flex-end; gap: 8px; }
  .error { color: var(--danger-color); margin: 0; font-size: var(--font-size-small); }
</style>
