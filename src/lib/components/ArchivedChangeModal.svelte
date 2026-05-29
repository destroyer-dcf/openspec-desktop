<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { marked } from "marked";
  import DOMPurify from "dompurify";
  import { X } from "lucide-svelte";
  import type { UiKey } from "$lib/i18n";

  export let changeName = "";
  export let documents: { name: string; path: string }[] = [];
  export let t: (key: UiKey) => string;
  export let onClose: () => void;

  let selectedPath: string | null = null;
  let selectedName = "";
  let content = "";
  let message = "";
  let loading = false;

  $: if (!selectedPath && documents.length > 0) {
    void selectDocument(documents[0]);
  }

  $: previewHtml = DOMPurify.sanitize(marked.parse(content) as string);

  async function selectDocument(doc: { name: string; path: string }) {
    selectedPath = doc.path;
    selectedName = doc.name;
    loading = true;
    message = "";
    try {
      content = await invoke<string>("read_file", { path: doc.path });
    } catch (error) {
      content = "";
      message = String(error);
    } finally {
      loading = false;
    }
  }
</script>

<div class="modal" role="dialog" aria-modal="true" aria-label={`${t("view")} ${changeName}`}>
  <div class="sheet">
    <header>
      <h3>{t("archived_query")}: {changeName}</h3>
      <button on:click={onClose} aria-label={t("close")} title={t("close")}><X size={14} /></button>
    </header>

    <div class="body">
      <aside>
        <h4>{t("documents")}</h4>
        {#if documents.length === 0}
          <p>{t("no_documents")}</p>
        {:else}
          <ul>
            {#each documents as doc}
              <li>
                <button
                  class:active={doc.path === selectedPath}
                  on:click={() => void selectDocument(doc)}
                  aria-label={`${t("view")} ${doc.name}`}
                  title={`${t("view")} ${doc.name}`}
                >
                  {doc.name}
                </button>
              </li>
            {/each}
          </ul>
        {/if}
      </aside>

      <section>
        <h4>{selectedName || t("document")}</h4>
        {#if loading}
          <p>{t("loading")}</p>
        {:else if message}
          <p>{message}</p>
        {:else}
          <article>{@html previewHtml}</article>
        {/if}
      </section>
    </div>
  </div>
</div>

<style>
  .modal { position: fixed; inset: 0; background: var(--overlay-backdrop); display: grid; place-items: center; z-index: 60; }
  .sheet {
    width: min(1000px, calc(100vw - 40px));
    height: min(700px, calc(100vh - 40px));
    overflow: hidden;
    border: 1px solid var(--border-default);
    border-radius: 10px;
    background: var(--bg-primary);
    display: grid;
    grid-template-rows: auto minmax(0, 1fr);
  }
  header { display: flex; align-items: center; justify-content: space-between; gap: 8px; padding: 10px; border-bottom: 1px solid var(--border-default); }
  h3, h4 { margin: 0; }
  .body { display: grid; grid-template-columns: minmax(220px, 280px) minmax(0, 1fr); min-height: 0; }
  aside { border-right: 1px solid var(--border-default); padding: 10px; min-width: 0; overflow: auto; }
  section { padding: 10px; min-height: 0; overflow: hidden; display: grid; grid-template-rows: auto minmax(0, 1fr); gap: 8px; }
  ul { list-style: none; margin: 8px 0 0 0; padding: 0; display: grid; gap: 6px; }
  li button { width: 100%; text-align: left; }
  li button.active { border-color: var(--accent-color); }
  article {
    border: 1px solid var(--border-default);
    border-radius: 8px;
    background: var(--bg-secondary);
    padding: 10px;
    font-size: var(--font-size-small);
    min-height: 0;
    overflow: auto;
  }
  p { color: var(--text-secondary); font-size: var(--font-size-small); }
</style>
