<script lang="ts">
  import { FolderOpen, Plus, Settings, X } from "lucide-svelte";
  import type { UiKey } from "$lib/i18n";
  import type { ProjectHandle } from "$lib/types";

  export let projects: ProjectHandle[] = [];
  export let activeIndex: number | null = null;
  export let onAdd: () => void;
  export let onSelect: (index: number) => void;
  export let onUnlink: (index: number) => void;
  export let onOpenSettings: () => void;
  export let t: (key: UiKey) => string;
</script>

<aside class="sidebar">
  <div class="body">
    <div class="head">
      <h2>{t("projects")}</h2>
      <button class="icon primary" on:click={onAdd} aria-label={t("add_project")} title={t("add_project")}>
        <Plus size={14} />
      </button>
    </div>

    {#if projects.length === 0}
      <p class="empty">{t("add_first_project")}</p>
    {:else}
      <ul>
        {#each projects as project, i}
          <li>
            <article class="card" class:active={activeIndex === i}>
              <button
                class="card-main"
                class:active={activeIndex === i}
                on:click={() => onSelect(i)}
                aria-label={`Activar ${project.name}`}
                title={`Activar ${project.name}`}
              >
                <div class="card-row">
                  <FolderOpen size={14} />
                  <strong>{project.name}</strong>
                </div>
                <span class="path" title={project.path}>{project.path}</span>
              </button>
              <button
                class="icon danger unlink"
                on:click={() => onUnlink(i)}
                aria-label={`Desvincular ${project.name}`}
                title={`Desvincular ${project.name}`}
              >
                <X size={13} />
              </button>
            </article>
          </li>
        {/each}
      </ul>
    {/if}
  </div>

  <footer class="footer">
    <button class="settings-btn" on:click={onOpenSettings} aria-label={t("open_settings")} title={t("open_settings")}>
      <Settings size={14} />
      {t("settings")}
    </button>
  </footer>
</aside>

<style>
  .sidebar {
    width: 100%;
    height: 100%;
    min-width: 0;
    box-sizing: border-box;
    border-right: 1px solid var(--border-default);
    padding: 10px;
    padding-top: 14px;
    background: var(--bg-secondary);
    color: var(--text-primary);
    min-height: 0;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }
  .body {
    flex: 1;
    min-height: 0;
    overflow: auto;
    padding-bottom: 8px;
  }
  .head { display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px; }
  h2 { margin: 0; font-size: var(--font-size-base); }
  ul { list-style: none; margin: 0; padding: 0; display: grid; gap: 6px; }
  .card {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    gap: 6px;
    padding: 7px;
    border: 1px solid var(--border-default);
    border-radius: 10px;
    background: color-mix(in srgb, var(--bg-secondary) 92%, transparent);
  }
  .card.active {
    border-color: var(--accent-color);
    background: color-mix(in srgb, var(--accent-color) 16%, var(--bg-secondary));
  }
  .card-main {
    width: 100%;
    min-width: 0;
    text-align: left;
    display: grid;
    gap: 4px;
    border: 0;
    padding: 0;
    background: transparent;
    color: var(--text-primary);
  }
  .card-main:hover {
    background: transparent;
  }
  .card-main.active {
    color: var(--text-primary);
  }
  .card-row {
    display: flex;
    align-items: center;
    gap: 6px;
    min-width: 0;
  }
  .card-row strong {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .path {
    font-size: var(--font-size-small);
    color: var(--text-secondary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .unlink {
    align-self: start;
  }
  .icon { width: 30px; justify-content: center; display: inline-flex; align-items: center; }
  .danger {
    color: var(--danger-color);
  }
  .footer {
    margin-top: auto;
    border-top: 1px solid var(--border-default);
    padding-top: 8px;
  }
  .settings-btn {
    width: 100%;
    display: inline-flex;
    align-items: center;
    justify-content: flex-start;
    text-align: left;
    gap: 6px;
    border: 0;
    background: transparent;
    color: var(--text-primary);
    padding: 8px 10px;
    border-radius: 8px;
  }
  .settings-btn:hover {
    background: color-mix(in srgb, var(--accent-color) 14%, var(--bg-secondary));
    box-shadow: inset 0 0 0 1px color-mix(in srgb, var(--accent-color) 55%, var(--border-default));
  }
  .settings-btn:focus-visible {
    outline: 2px solid var(--accent-color);
    outline-offset: 1px;
  }
  .empty { color: var(--text-secondary); font-size: var(--font-size-small); }

</style>
