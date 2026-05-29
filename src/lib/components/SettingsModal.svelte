<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { Palette, Type, X } from "lucide-svelte";
  import type { UiKey } from "$lib/i18n";
  import type { ThemeMode, FontScale, CardColorChoice, CardColorPrefs, UiLanguage } from "$lib/ui-prefs";

  export let theme: ThemeMode = "light";
  export let fontScale: FontScale = "normal";
  export let language: UiLanguage = "es";
  export let cardColors: CardColorPrefs;
  export let t: (key: UiKey) => string;
  export let onClose: () => void;
  export let onThemeChange: (theme: ThemeMode) => void;
  export let onFontScaleChange: (fontScale: FontScale) => void;
  export let onLanguageChange: (language: UiLanguage) => void;
  export let onCardColorsChange: (cardColors: CardColorPrefs) => void;

  let appVersion = "Cargando...";
  let openspecVersion = "Cargando...";

  onMount(async () => {
    try {
      const versions = await invoke<{ app_version: string; openspec_version: string }>("get_versions");
      appVersion = versions.app_version || "No disponible";
      openspecVersion = versions.openspec_version || t("version_unavailable");
    } catch {
      appVersion = t("version_unavailable");
      openspecVersion = t("version_unavailable");
    }
  });

  const cardColorOptions: Array<{ value: CardColorChoice; label: string }> = [
    { value: "none", label: "Sin color" },
    { value: "green", label: "Verde (Completado)" },
    { value: "red", label: "Rojo (Error/Fallo)" },
    { value: "yellow", label: "Amarillo (Pendiente)" },
    { value: "blue", label: "Azul (En progreso)" },
    { value: "gray", label: "Gris (Pausado)" },
    { value: "orange", label: "Naranja (Advertencia)" },
  ];

  function updateCardColor<K extends keyof CardColorPrefs>(key: K, value: CardColorPrefs[K]) {
    onCardColorsChange({ ...cardColors, [key]: value });
  }

  function onCardColorChange<K extends keyof CardColorPrefs>(key: K, event: Event) {
    const value = (event.currentTarget as HTMLSelectElement).value as CardColorPrefs[K];
    updateCardColor(key, value);
  }
</script>

<div class="modal" role="dialog" aria-modal="true" aria-label={t("settings")}>
  <div class="card">
    <h3>{t("settings")}</h3>

    <div class="versions-panel" aria-label="Versiones">
      <h4>{t("versions")}</h4>
      <div class="version-row">
        <span>{t("cli_version")}</span>
        <code>{openspecVersion}</code>
      </div>
      <div class="version-row">
        <span>{t("app_version")}</span>
        <code>{appVersion}</code>
      </div>
    </div>

    <div class="row">
      <label for="theme-select"><Palette size={14} /> {t("theme")}</label>
      <select id="theme-select" bind:value={theme} on:change={() => onThemeChange(theme)} aria-label={t("theme")}>
        <option value="light">Light</option>
        <option value="dark">Dark</option>
      </select>
    </div>

    <div class="row">
      <label for="font-scale-select"><Type size={14} /> {t("text_size")}</label>
      <select
        id="font-scale-select"
        bind:value={fontScale}
        on:change={() => onFontScaleChange(fontScale)}
        aria-label={t("text_size")}
      >
        <option value="compact">Compact</option>
        <option value="normal">Normal</option>
      </select>
    </div>

    <div class="row">
      <label for="language-select"><Type size={14} /> {t("language")}</label>
      <select
        id="language-select"
        bind:value={language}
        on:change={() => onLanguageChange(language)}
        aria-label={t("language")}
      >
        <option value="es">Español</option>
        <option value="en">English</option>
        <option value="fr">Français</option>
        <option value="de">Deutsch</option>
        <option value="pt">Português</option>
      </select>
    </div>

    <div class="group">
      <h4><Palette size={14} /> {t("card_colors_panel")}</h4>

      <div class="row two">
        <label for="active-done-select">{t("active_done")}</label>
        <select
          id="active-done-select"
          value={cardColors.activeDone}
          on:change={(event) => onCardColorChange("activeDone", event)}
          aria-label="Color para cambios activos completados"
        >
          {#each cardColorOptions as option}
            <option value={option.value}>{option.label}</option>
          {/each}
        </select>
      </div>

      <div class="row two">
        <label for="active-pending-select">{t("active_pending")}</label>
        <select
          id="active-pending-select"
          value={cardColors.activePending}
          on:change={(event) => onCardColorChange("activePending", event)}
          aria-label="Color para cambios activos pendientes"
        >
          {#each cardColorOptions as option}
            <option value={option.value}>{option.label}</option>
          {/each}
        </select>
      </div>

      <div class="row two">
        <label for="proposal-feature-select">{t("proposal_feature")}</label>
        <select
          id="proposal-feature-select"
          value={cardColors.proposalFeature}
          on:change={(event) => onCardColorChange("proposalFeature", event)}
          aria-label="Color para propuestas feature"
        >
          {#each cardColorOptions as option}
            <option value={option.value}>{option.label}</option>
          {/each}
        </select>
      </div>

      <div class="row two">
        <label for="proposal-bug-select">{t("proposal_bug")}</label>
        <select
          id="proposal-bug-select"
          value={cardColors.proposalBug}
          on:change={(event) => onCardColorChange("proposalBug", event)}
          aria-label="Color para propuestas bug"
        >
          {#each cardColorOptions as option}
            <option value={option.value}>{option.label}</option>
          {/each}
        </select>
      </div>

      <div class="row two">
        <label for="archived-select">{t("archived")}</label>
        <select
          id="archived-select"
          value={cardColors.archived}
          on:change={(event) => onCardColorChange("archived", event)}
          aria-label="Color para cambios archivados"
        >
          {#each cardColorOptions as option}
            <option value={option.value}>{option.label}</option>
          {/each}
        </select>
      </div>
    </div>

    <div class="actions">
      <button on:click={onClose} title={t("close")} aria-label={t("close")}><X size={14} /> {t("close")}</button>
    </div>
  </div>
</div>

<style>
  .modal { position: fixed; inset: 0; background: var(--overlay-backdrop); display: grid; place-items: center; z-index: 50; }
  .card {
    width: min(460px, 92vw);
    background: var(--bg-secondary);
    color: var(--text-primary);
    border: 1px solid var(--border-default);
    border-radius: 8px;
    padding: 14px;
  }
  .row { display: grid; gap: 6px; margin-top: 10px; }
  .versions-panel {
    border: 1px solid var(--border-default);
    border-radius: 8px;
    background: var(--bg-primary);
    padding: 10px;
    margin-top: 8px;
    display: grid;
    gap: 8px;
  }
  .version-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 10px;
    color: var(--text-secondary);
    font-size: var(--font-size-small);
  }
  .version-row code {
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
    background: var(--bg-secondary);
    border: 1px solid var(--border-default);
    border-radius: 6px;
    color: var(--text-primary);
    padding: 2px 6px;
    max-width: 60%;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .row.two { grid-template-columns: minmax(0, 1fr) 180px; align-items: center; }
  label { color: var(--text-secondary); font-size: var(--font-size-small); display: inline-flex; align-items: center; gap: 6px; }
  .group { margin-top: 12px; border-top: 1px solid var(--border-default); padding-top: 10px; display: grid; gap: 6px; }
  h4 { margin: 0; font-size: var(--font-size-base); display: inline-flex; align-items: center; gap: 6px; }
  select { background: var(--bg-secondary); color: var(--text-primary); border: 1px solid var(--border-default); border-radius: 6px; padding: 6px 8px; }
  select:focus { outline: 2px solid var(--focus-ring); outline-offset: 1px; }
  .actions { display: flex; justify-content: flex-end; margin-top: 12px; }
  .actions button { display: inline-flex; align-items: center; gap: 6px; }
</style>
