<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onDestroy, onMount } from "svelte";
  import Sidebar from "$lib/components/Sidebar.svelte";
  import Dashboard from "$lib/components/Dashboard.svelte";
  import ChangeDetail from "$lib/components/ChangeDetail.svelte";
  import EditorMarkdown from "$lib/components/EditorMarkdown.svelte";
  import WizardInit from "$lib/components/WizardInit.svelte";
  import SettingsModal from "$lib/components/SettingsModal.svelte";
  import { createTranslator } from "$lib/i18n";
  import { DEFAULT_CARD_COLORS } from "$lib/ui-prefs";
  import type { ThemeMode, FontScale, CardColorChoice, CardColorPrefs, UiLanguage } from "$lib/ui-prefs";
  import type {
    Artifact,
    Change,
    InitProjectInput,
    OpenProjectResponse,
    ProjectHandle,
    ProjectState,
  } from "$lib/types";

  const PREFS_KEY = "openspec-manager-ui-prefs";

  let projects: ProjectHandle[] = [];
  let activeIndex: number | null = null;
  let state: ProjectState | null = null;
  let selectedChange: Change | null = null;
  let selectedArtifact: Artifact | null = null;
  let error = "";

  let wizardPath: string | null = null;
  let wizardBusy = false;
  let wizardError = "";

  let settingsOpen = false;
  let theme: ThemeMode = "light";
  let fontScale: FontScale = "normal";
  let language: UiLanguage = "es";
  let cardColors: CardColorPrefs = { ...DEFAULT_CARD_COLORS };
  let sidebarWidth = 320;
  let isResizingSidebar = false;

  function normalizeCardColor(value: unknown): CardColorChoice {
    if (value === "blue" || value === "green" || value === "red" || value === "yellow" || value === "gray" || value === "orange") return value;
    return "none";
  }

  function normalizeCardColors(value: unknown): CardColorPrefs {
    const parsed = (value ?? {}) as Partial<CardColorPrefs>;
    return {
      activeDone: normalizeCardColor(parsed.activeDone),
      activePending: normalizeCardColor(parsed.activePending),
      proposalFeature: normalizeCardColor(parsed.proposalFeature),
      proposalBug: normalizeCardColor(parsed.proposalBug),
      archived: normalizeCardColor(parsed.archived),
    };
  }

  $: t = createTranslator(language);

  function normalizeLanguage(value: unknown): UiLanguage {
    return value === "en" || value === "fr" || value === "de" || value === "pt" ? value : "es";
  }

  function applyUiPrefs(next: { theme?: ThemeMode; fontScale?: FontScale; cardColors?: CardColorPrefs; language?: UiLanguage }) {
    if (next.theme) theme = next.theme;
    if (next.fontScale) fontScale = next.fontScale;
    if (next.cardColors) cardColors = normalizeCardColors(next.cardColors);
    if (next.language) language = normalizeLanguage(next.language);

    document.documentElement.setAttribute("data-theme", theme);
    document.documentElement.setAttribute("data-font-scale", fontScale);

    localStorage.setItem(PREFS_KEY, JSON.stringify({ theme, fontScale, cardColors, language }));
  }

  function restoreUiPrefs() {
    const raw = localStorage.getItem(PREFS_KEY);
    if (!raw) {
      applyUiPrefs({
        theme: "light",
        fontScale: "normal",
        cardColors: DEFAULT_CARD_COLORS,
      });
      return;
    }

    try {
      const parsed = JSON.parse(raw) as Partial<{ theme: ThemeMode; fontScale: FontScale; cardColors: CardColorPrefs; language: UiLanguage }>;
      applyUiPrefs({
        theme: parsed.theme === "dark" ? "dark" : "light",
        fontScale: parsed.fontScale === "compact" ? "compact" : "normal",
        cardColors: normalizeCardColors(parsed.cardColors),
        language: normalizeLanguage(parsed.language),
      });
    } catch {
      applyUiPrefs({
        theme: "light",
        fontScale: "normal",
        cardColors: DEFAULT_CARD_COLORS,
      });
    }
  }

  async function loadProjectsFromBackend() {
    projects = await invoke<ProjectHandle[]>("get_projects");
    activeIndex = await invoke<number | null>("get_active_index");
    state = await invoke<ProjectState | null>("get_state");
  }

  async function addProject() {
    error = "";
    const folder = await invoke<string | null>("pick_project_folder");
    if (!folder) return;

    const response = await invoke<OpenProjectResponse>("open_project", { path: folder });
    if (response.status === "loaded") {
      await loadProjectsFromBackend();
      return;
    }

    wizardPath = response.path;
    wizardError = "";
  }

  async function selectProject(index: number) {
    await invoke("set_active_project", { index });
    await loadProjectsFromBackend();
    selectedChange = null;
    selectedArtifact = null;
  }

  async function unlinkProject(index: number) {
    await invoke("unlink_project", { index });
    await loadProjectsFromBackend();
    selectedChange = null;
    selectedArtifact = null;
  }

  async function onWizardSubmit(payload: InitProjectInput) {
    wizardBusy = true;
    wizardError = "";
    try {
      const cliOk = await invoke<boolean>("check_openspec_cli");
      if (!cliOk) {
        wizardError = t("cli_missing");
        return;
      }
      await invoke("init_project", { input: payload });
      wizardPath = null;
      await loadProjectsFromBackend();
    } catch (e) {
      wizardError = String(e);
    } finally {
      wizardBusy = false;
    }
  }

  function onSelectChange(change: Change) {
    selectedChange = change;
    selectedArtifact = null;
  }

  function onOpenArtifact(artifact: Artifact) {
    selectedArtifact = artifact;
  }

  function onBackToDashboard() {
    selectedChange = null;
    selectedArtifact = null;
  }

  onMount(async () => {
    restoreUiPrefs();
    await loadProjectsFromBackend();
  });

  const unlistenPromise = listen<string>("project-updated", async (event) => {
    const path = event.payload;
    await invoke("open_project", { path });
    await loadProjectsFromBackend();
  });

  onDestroy(() => {
    void unlistenPromise.then((fn) => fn());
    if (typeof window !== "undefined") {
      window.removeEventListener("mousemove", onSidebarResizeMove);
      window.removeEventListener("mouseup", stopSidebarResize);
    }
  });

  function startSidebarResize() {
    if (typeof window === "undefined") return;
    isResizingSidebar = true;
    document.body.style.userSelect = "none";
    window.addEventListener("mousemove", onSidebarResizeMove);
    window.addEventListener("mouseup", stopSidebarResize);
  }

  function onSidebarResizeMove(event: MouseEvent) {
    if (!isResizingSidebar || typeof window === "undefined") return;
    const min = 260;
    const max = Math.max(420, Math.floor(window.innerWidth * 0.6));
    const next = Math.min(max, Math.max(min, event.clientX));
    sidebarWidth = next;
  }

  function stopSidebarResize() {
    isResizingSidebar = false;
    document.body.style.userSelect = "";
    if (typeof window === "undefined") return;
    window.removeEventListener("mousemove", onSidebarResizeMove);
    window.removeEventListener("mouseup", stopSidebarResize);
  }
</script>

<main class="layout">
  <div class="sidebar-shell" style={`width: ${sidebarWidth}px`}>
    <Sidebar
      {projects}
      {activeIndex}
      {t}
      onAdd={addProject}
      onSelect={selectProject}
      onUnlink={unlinkProject}
      onOpenSettings={() => (settingsOpen = true)}
    />
  </div>
  <button
    type="button"
    class="sidebar-resizer"
    aria-label="Redimensionar panel de proyectos"
    on:mousedown={startSidebarResize}
  ></button>

  <section class="content">
    <div class="content-head">
      <div class="app-brand" aria-label="OpenSpec Desktop">
        <div class="app-brand-text">
          <strong>OpenSpec</strong>
          <span>Desktop</span>
        </div>
      </div>
    </div>

    {#if error}<p class="error">{error}</p>{/if}

    {#if selectedChange && selectedArtifact}
      <EditorMarkdown path={selectedArtifact.path} {t} onClose={() => (selectedArtifact = null)} />
    {:else if selectedChange}
      <ChangeDetail change={selectedChange} {t} onOpenArtifact={onOpenArtifact} onBack={onBackToDashboard} />
    {:else}
      {#key JSON.stringify(cardColors)}
        <Dashboard {state} {cardColors} {t} {language} onSelectChange={onSelectChange} />
      {/key}
    {/if}
  </section>
</main>

{#if wizardPath}
  <WizardInit
    path={wizardPath}
    busy={wizardBusy}
    error={wizardError}
    {t}
    onCancel={() => {
      wizardPath = null;
      wizardError = "";
    }}
    onSubmit={onWizardSubmit}
  />
{/if}

{#if settingsOpen}
  <SettingsModal
    {theme}
    {fontScale}
    {language}
    {cardColors}
    {t}
    onClose={() => (settingsOpen = false)}
    onThemeChange={(next) => applyUiPrefs({ theme: next })}
    onFontScaleChange={(next) => applyUiPrefs({ fontScale: next })}
    onLanguageChange={(next) => applyUiPrefs({ language: next })}
    onCardColorsChange={(next) => applyUiPrefs({ cardColors: next })}
  />
{/if}

<style>
  :global(:root) {
    --bg-primary: #f6f8fa;
    --bg-secondary: #ffffff;
    --text-primary: #1f2328;
    --text-secondary: #59636e;
    --border-default: #d0d7de;
    --accent-color: #0969da;
    --focus-ring: #1f6feb;
    --danger-color: #cf222e;
    --button-bg: #f6f8fa;
    --button-hover: #eef2f6;
    --text-on-accent: #ffffff;
    --overlay-backdrop: rgba(31, 35, 40, 0.45);
    --font-size-base: 14px;
    --font-size-small: 12px;
    --button-accent: #0969da;
    --button-accent-hover: #0550ae;
    --button-accent-text: #ffffff;
    --window-top-inset: 3px;
  }

  :global(:root[data-theme="dark"]) {
    --bg-primary: #0d1117;
    --bg-secondary: #161b22;
    --text-primary: #e6edf3;
    --text-secondary: #9da7b3;
    --border-default: #30363d;
    --accent-color: #2f81f7;
    --focus-ring: #2f81f7;
    --danger-color: #f85149;
    --button-bg: #21262d;
    --button-hover: #30363d;
    --text-on-accent: #ffffff;
    --overlay-backdrop: rgba(1, 4, 9, 0.62);
    --button-accent: #2f81f7;
    --button-accent-hover: #1f6feb;
    --button-accent-text: #ffffff;
  }

  :global(:root[data-font-scale="compact"]) {
    --font-size-base: 13px;
    --font-size-small: 11px;
  }

  :global(:root[data-font-scale="normal"]) {
    --font-size-base: 14px;
    --font-size-small: 12px;
  }

  :global(html), :global(body) {
    margin: 0;
    height: 100%;
    overflow: hidden;
    background: var(--bg-primary);
    color: var(--text-primary);
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif;
    font-size: var(--font-size-base);
  }

  :global(button), :global(input), :global(select), :global(textarea) {
    font: inherit;
    color: var(--text-primary);
    background: var(--bg-secondary);
    border: 1px solid var(--border-default);
    border-radius: 6px;
    padding: 6px 8px;
  }

  :global(button) { background: var(--button-bg); cursor: pointer; }
  :global(button:hover) { background: var(--button-hover); }
  :global(button.primary) {
    background: var(--button-accent);
    border-color: var(--button-accent);
    color: var(--button-accent-text);
  }
  :global(button.primary:hover) { background: var(--button-accent-hover); }
  :global(button:focus), :global(input:focus), :global(select:focus), :global(textarea:focus) {
    outline: 2px solid var(--focus-ring);
    outline-offset: 1px;
  }

  .layout {
    display: flex;
    height: 100vh;
    min-height: 0;
    box-sizing: border-box;
    background: var(--bg-primary);
    overflow: hidden;
    position: relative;
    padding-top: var(--window-top-inset);
  }
  .layout::before {
    content: "";
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 1px;
    background: var(--border-default);
    pointer-events: none;
    z-index: 5;
  }

  .sidebar-shell {
    flex: 0 0 auto;
    min-width: 260px;
    height: 100%;
    min-height: 0;
    overflow: hidden;
  }

  .sidebar-resizer {
    border: 0;
    padding: 0;
    background: transparent;
    width: 8px;
    cursor: col-resize;
    flex: 0 0 auto;
    position: relative;
  }

  .sidebar-resizer::before {
    content: "";
    position: absolute;
    top: 0;
    bottom: 0;
    left: 3px;
    width: 1px;
    background: var(--border-default);
  }

  .sidebar-resizer:hover::before {
    background: var(--accent-color);
  }

  .content {
    flex: 1 1 auto;
    padding: 12px;
    color: var(--text-primary);
    min-width: 0;
    min-height: 0;
    overflow: auto;
  }
  .content-head { display: flex; align-items: center; gap: 8px; margin-bottom: 10px; }
  .app-brand {
    display: inline-flex;
    align-items: center;
    gap: 0;
  }
  .app-brand-text {
    display: grid;
    line-height: 1;
    gap: 6px;
  }
  .app-brand-text strong {
    font-size: 44px;
    font-weight: 800;
    letter-spacing: -0.02em;
    color: var(--text-primary);
  }
  .app-brand-text span {
    font-size: 24px;
    font-weight: 700;
    color: var(--accent-color);
  }
  .error { color: var(--danger-color); font-size: var(--font-size-small); }
</style>
