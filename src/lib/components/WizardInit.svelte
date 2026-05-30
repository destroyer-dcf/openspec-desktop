<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
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
  const AI_TOOL_OPTIONS = [
    { value: "amazon-q", label: "Amazon Q Developer" },
    { value: "antigravity", label: "Antigravity" },
    { value: "auggie", label: "Auggie (Augment CLI)" },
    { value: "bob", label: "Bob Shell" },
    { value: "claude", label: "Claude Code" },
    { value: "cline", label: "Cline" },
    { value: "codex", label: "Codex" },
    { value: "forgecode", label: "ForgeCode" },
    { value: "codebuddy", label: "CodeBuddy Code (CLI)" },
    { value: "continue", label: "Continue" },
    { value: "costrict", label: "CoStrict" },
    { value: "crush", label: "Crush" },
    { value: "cursor", label: "Cursor" },
    { value: "factory", label: "Factory Droid" },
    { value: "gemini", label: "Gemini CLI" },
    { value: "github-copilot", label: "GitHub Copilot" },
    { value: "iflow", label: "iFlow" },
    { value: "junie", label: "Junie" },
    { value: "kilocode", label: "Kilo Code" },
    { value: "kiro", label: "Kiro" },
    { value: "opencode", label: "OpenCode" },
    { value: "pi", label: "Pi" },
    { value: "qoder", label: "Qoder" },
    { value: "lingma", label: "Lingma" },
    { value: "qwen", label: "Qwen Code" },
    { value: "roocode", label: "RooCode" },
    { value: "trae", label: "Trae" },
    { value: "windsurf", label: "Windsurf" },
  ];
  let aiProviders: string[] = ["codex"];
  let schema = "spec-driven";
  let schemaOptions: string[] = ["spec-driven"];
  type SchemaRules = { proposal: string[]; specs: string[]; design: string[]; tasks: string[] };
  let schemaRulesMap: Record<string, SchemaRules> = {};
  const SPEC_DRIVEN_RULES_ES = {
    proposal: [
      "Mantén la propuesta en menos de 500 palabras salvo que el cambio sea excepcionalmente amplio.",
      "Cubre solo el por qué, qué cambia, alcance, no-objetivos, capacidades e impacto.",
      "Mueve las decisiones de implementación y detalles a nivel de fichero al diseño.",
    ],
    specs: [
      "Trata las specs como el contrato de comportamiento, no como un plan de implementación.",
      "Incluye solo comportamiento observable, resultados para el usuario y contratos externos.",
      "Excluye estructura de código, rutas de ficheros, componentes, comandos y decisiones de arquitectura.",
    ],
    design: [
      "Mantén las secciones de diseño concisas y orientadas a decisiones.",
      "Incluye ficheros concretos, rutas, assets, dependencias o integraciones solo cuando clarifiquen la implementación.",
      "No repitas la motivación de la propuesta ni los requisitos de las specs.",
    ],
    tasks: [
      "Escribe cada tarea como un entregable concreto.",
      "Incluye tareas de validación para el comportamiento modificado.",
      "Evita reiterar la propuesta, las specs o el diseño en el texto de las tareas.",
    ],
  };
  let proposalRules = SPEC_DRIVEN_RULES_ES.proposal.join("\n");
  let specsRules = SPEC_DRIVEN_RULES_ES.specs.join("\n");
  let designRules = SPEC_DRIVEN_RULES_ES.design.join("\n");
  let tasksRules = SPEC_DRIVEN_RULES_ES.tasks.join("\n");
  let lastRuleSeed = "";
  let step = 1;

  $: if (path) {
    void loadSchemas();
  }

  async function loadSchemas() {
    try {
      const result = await invoke<{
        schemas: string[];
        templates?: Array<{
          schema: string;
          suggested_rules?: SchemaRules;
        }>;
      }>("list_schemas");
      const options = (result.schemas ?? []).filter((s) => s && s.trim().length > 0);
      schemaOptions = options.length > 0 ? options : ["spec-driven"];
      schemaRulesMap = {};
      for (const template of result.templates ?? []) {
        if (!template?.schema || !template.suggested_rules) continue;
        schemaRulesMap[template.schema] = template.suggested_rules;
      }
      if (!schemaOptions.includes(schema)) {
        schema = schemaOptions[0];
      }
      applyDefaultRules();
    } catch {
      schemaOptions = ["spec-driven"];
      schema = "spec-driven";
      applyDefaultRules();
    }
  }

  function defaultRulesFor(schemaValue: string, languageValue: string) {
    const lower = (languageValue || "").toLowerCase();
    const isSpanish = lower.includes("espa") || lower.includes("spanish") || lower === "es";
    if (schemaValue === "spec-driven") {
      return isSpanish
        ? SPEC_DRIVEN_RULES_ES
        : {
            proposal: [
              "Keep the proposal under 500 words unless the change is exceptionally broad.",
              "Cover only why, what changes, scope, non-goals, capabilities, and impact.",
              "Move implementation decisions and file-level details to design.",
            ],
            specs: [
              "Treat specs as the behavior contract, not an implementation plan.",
              "Include only observable behavior, user outcomes, and external contracts.",
              "Exclude code structure, file paths, components, commands, and architecture decisions.",
            ],
            design: [
              "Keep design sections concise and decision-oriented.",
              "Include concrete files, paths, assets, dependencies, or integrations only when they clarify implementation.",
              "Do not repeat proposal motivation or specs requirements.",
            ],
            tasks: [
              "Write each task as a concrete deliverable.",
              "Include validation tasks for modified behavior.",
              "Avoid repeating proposal, specs, or design text in tasks.",
            ],
          };
    }

    const dynamic = schemaRulesMap[schemaValue];
    if (dynamic) {
      return {
        proposal: dynamic.proposal ?? [],
        specs: dynamic.specs ?? [],
        design: dynamic.design ?? [],
        tasks: dynamic.tasks ?? [],
      };
    }

    return { proposal: [], specs: [], design: [], tasks: [] };
  }

  function applyDefaultRules() {
    const defaults = defaultRulesFor(schema, language);
    proposalRules = defaults.proposal.join("\n");
    specsRules = defaults.specs.join("\n");
    designRules = defaults.design.join("\n");
    tasksRules = defaults.tasks.join("\n");
    lastRuleSeed = `${schema}|${language}`;
  }

  function parseRulesBlock(raw: string) {
    return raw
      .split("\n")
      .map((line) => line.trim())
      .filter((line) => line.length > 0);
  }

  $: canGoNext = name.trim().length > 0 && stack.trim().length > 0 && !busy;
  $: canSubmit = canGoNext && aiProviders.length > 0 && !busy;

  function toggleProvider(provider: string, checked: boolean) {
    if (checked) {
      if (!aiProviders.includes(provider)) aiProviders = [...aiProviders, provider];
      return;
    }
    aiProviders = aiProviders.filter((p) => p !== provider);
  }

  function submit() {
    if (!canSubmit) return;
    onSubmit({
      path,
      name,
      schema,
      language,
      audience,
      domain,
      description,
      stack,
      architecture,
      deployment_flow: deploymentFlow,
      ai_provider: aiProviders,
      proposal_rules: parseRulesBlock(proposalRules),
      specs_rules: parseRulesBlock(specsRules),
      design_rules: parseRulesBlock(designRules),
      tasks_rules: parseRulesBlock(tasksRules),
    });
  }

  $: if (schema && language && `${schema}|${language}` !== lastRuleSeed) {
    applyDefaultRules();
  }
</script>

<div class="modal">
  <div class="card">
    <h3>{t("init_project")}</h3>
    <p class="path">{path}</p>
    <p class="step-indicator">Paso {step} de 2</p>
    {#if step === 1}
      <div class="grid">
        <input placeholder="Nombre*" bind:value={name} />
        <input placeholder="Idioma" bind:value={language} />
        <input placeholder="Audiencia" bind:value={audience} />
        <input placeholder="Dominio" bind:value={domain} />
        <input placeholder="Stack*" bind:value={stack} />
        <select bind:value={schema}>
          {#each schemaOptions as option}
            <option value={option}>{option}</option>
          {/each}
        </select>
        <input placeholder="Architecture (opcional)" bind:value={architecture} />
        <input placeholder="Deployment flow (opcional)" bind:value={deploymentFlow} />
        <textarea placeholder="Descripción" bind:value={description}></textarea>
        <label class="rule-label" for="proposal-rules">Rules: proposal</label>
        <textarea id="proposal-rules" placeholder="Rules: proposal (una línea por regla)" bind:value={proposalRules}></textarea>
        <label class="rule-label" for="specs-rules">Rules: specs</label>
        <textarea id="specs-rules" placeholder="Rules: specs (una línea por regla)" bind:value={specsRules}></textarea>
        <label class="rule-label" for="design-rules">Rules: design</label>
        <textarea id="design-rules" placeholder="Rules: design (una línea por regla)" bind:value={designRules}></textarea>
        <label class="rule-label" for="tasks-rules">Rules: tasks</label>
        <textarea id="tasks-rules" placeholder="Rules: tasks (una línea por regla)" bind:value={tasksRules}></textarea>
      </div>
    {:else}
      <div class="grid">
        <div class="rule-label">Proveedor IA* (multi)</div>
        <div class="provider-scroll">
          <div class="provider-options">
            {#each AI_TOOL_OPTIONS as option}
              <label class:provider-selected={aiProviders.includes(option.value)}>
                <input
                  type="checkbox"
                  checked={aiProviders.includes(option.value)}
                  on:change={(e) => toggleProvider(option.value, (e.currentTarget as HTMLInputElement).checked)}
                />
                <span>{option.label}</span>
              </label>
            {/each}
          </div>
        </div>
      </div>
    {/if}
    {#if error}<p class="error">{error}</p>{/if}
    <div class="actions">
      <button on:click={onCancel} disabled={busy} aria-label={t("cancel")} title={t("cancel")}><X size={14} /> {t("cancel")}</button>
      {#if step === 1}
        <button class="primary" on:click={() => (step = 2)} disabled={!canGoNext} aria-label="Siguiente" title="Siguiente">
          Siguiente
        </button>
      {:else}
        <button on:click={() => (step = 1)} disabled={busy} aria-label="Atrás" title="Atrás">Atrás</button>
        <button class="primary" on:click={submit} disabled={!canSubmit} aria-label={t("confirm")} title={t("confirm")}>
          <Check size={14} /> {busy ? t("initializing") : t("confirm")}
        </button>
      {/if}
    </div>
  </div>
</div>

<style>
  .modal { position: fixed; inset: 0; background: var(--overlay-backdrop); display: grid; place-items: center; }
  .card { width: min(760px, 92vw); max-height: 88vh; overflow: auto; background: var(--bg-secondary); color: var(--text-primary); border: 1px solid var(--border-default); border-radius: 10px; padding: 12px; }
  .card:has(.provider-options) { width: min(980px, 96vw); }
  .path { color: var(--text-secondary); font-size: var(--font-size-small); }
  .step-indicator { color: var(--text-secondary); font-size: var(--font-size-small); margin-bottom: 8px; font-weight: 700; }
  .grid { display: grid; grid-template-columns: 1fr 1fr; gap: 8px; }
  .rule-label { grid-column: 1 / -1; font-size: var(--font-size-small); color: var(--text-secondary); font-weight: 700; }
  .provider-scroll {
    grid-column: 1 / -1;
    max-height: none;
    overflow: visible;
    border: 1px solid var(--border-default);
    border-radius: 8px;
    padding: 8px;
    background: var(--bg-secondary);
  }
  .provider-options {
    display: grid;
    grid-template-columns: repeat(4, minmax(0, 1fr));
    gap: 8px;
  }
  .provider-options label {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: var(--font-size-small);
    border: 1px solid var(--border-default);
    border-radius: 8px;
    padding: 8px 10px;
    background: var(--bg-tertiary);
    cursor: pointer;
    min-height: 40px;
  }
  .provider-options label:hover {
    border-color: var(--border-strong);
    background: var(--bg-hover);
  }
  .provider-options label.provider-selected {
    border-color: var(--primary-color);
    background: var(--primary-soft);
  }
  .provider-options input {
    margin: 0;
    flex: 0 0 auto;
  }
  .provider-options span {
    line-height: 1.2;
  }
  @media (max-width: 1200px) {
    .provider-options { grid-template-columns: repeat(3, minmax(0, 1fr)); }
  }
  @media (max-width: 900px) {
    .provider-options { grid-template-columns: repeat(2, minmax(0, 1fr)); }
  }
  textarea { grid-column: 1 / -1; min-height: 80px; }
  select { grid-column: 1 / -1; }
  .actions { display: flex; justify-content: flex-end; gap: 8px; margin-top: 10px; }
  .actions button { display: inline-flex; align-items: center; gap: 6px; }
  .error { color: var(--danger-color); font-size: var(--font-size-small); }
  @media (max-width: 760px) { .grid { grid-template-columns: 1fr; } }
</style>
