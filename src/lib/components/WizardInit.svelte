<script lang="ts">
  import { Check, X } from "lucide-svelte";
  import type { UiKey } from "$lib/i18n";
  import type { InitProjectInput } from "$lib/types";

  export let path: string;
  export let busy = false;
  export let error = "";
  export let t: (key: UiKey) => string;
  export let onCancel: () => void;
  export let onSubmit: (payload: InitProjectInput) => void;

  let name = "";
  let language = "Español";
  let audience = "";
  let domain = "";
  let description = "";
  let stack = "";
  let architecture = "";
  let deploymentFlow = "";
  let aiProvider = "";

  $: canSubmit = name.trim().length > 0 && stack.trim().length > 0 && aiProvider.trim().length > 0 && !busy;

  function submit() {
    if (!canSubmit) return;
    onSubmit({
      path,
      name,
      language,
      audience,
      domain,
      description,
      stack,
      architecture,
      deployment_flow: deploymentFlow,
      ai_provider: aiProvider,
    });
  }
</script>

<div class="modal">
  <div class="card">
    <h3>{t("init_project")}</h3>
    <p class="path">{path}</p>
    <div class="grid">
      <input placeholder="Nombre*" bind:value={name} />
      <input placeholder="Idioma" bind:value={language} />
      <input placeholder="Audiencia" bind:value={audience} />
      <input placeholder="Dominio" bind:value={domain} />
      <input placeholder="Stack*" bind:value={stack} />
      <input placeholder="Architecture (opcional)" bind:value={architecture} />
      <input placeholder="Deployment flow (opcional)" bind:value={deploymentFlow} />
      <textarea placeholder="Descripción" bind:value={description}></textarea>
      <select bind:value={aiProvider}>
        <option value="">Proveedor IA*</option>
        <option value="Codex">Codex</option>
        <option value="Copilot">Copilot</option>
        <option value="OpenCode">OpenCode</option>
      </select>
    </div>
    {#if error}<p class="error">{error}</p>{/if}
    <div class="actions">
      <button on:click={onCancel} disabled={busy} aria-label={t("cancel")} title={t("cancel")}><X size={14} /> {t("cancel")}</button>
      <button class="primary" on:click={submit} disabled={!canSubmit} aria-label={t("confirm")} title={t("confirm")}>
        <Check size={14} /> {busy ? t("initializing") : t("confirm")}
      </button>
    </div>
  </div>
</div>

<style>
  .modal { position: fixed; inset: 0; background: var(--overlay-backdrop); display: grid; place-items: center; }
  .card { width: min(760px, 92vw); background: var(--bg-secondary); color: var(--text-primary); border: 1px solid var(--border-default); border-radius: 10px; padding: 12px; }
  .path { color: var(--text-secondary); font-size: var(--font-size-small); }
  .grid { display: grid; grid-template-columns: 1fr 1fr; gap: 8px; }
  textarea { grid-column: 1 / -1; min-height: 80px; }
  select { grid-column: 1 / -1; }
  .actions { display: flex; justify-content: flex-end; gap: 8px; margin-top: 10px; }
  .actions button { display: inline-flex; align-items: center; gap: 6px; }
  .error { color: var(--danger-color); font-size: var(--font-size-small); }
  @media (max-width: 760px) { .grid { grid-template-columns: 1fr; } }
</style>
