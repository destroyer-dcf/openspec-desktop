<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { marked } from "marked";
  import DOMPurify from "dompurify";
  import { ArrowLeft, Plus, Save, Undo2 } from "lucide-svelte";
  import type { UiKey } from "$lib/i18n";

  export let path: string | null = null;
  export let t: (key: UiKey) => string;
  export let onClose: () => void;

  type SpecDocument = { name: string; path: string };

  let original = "";
  let content = "";
  let message = "";
  let previewHtml = "";
  let specsMode = false;
  let specsDir = "";
  let specsDocs: SpecDocument[] = [];
  let selectedSpecPath: string | null = null;
  let creatingSpec = false;
  let newCapability = "";
  let createError = "";

  $: previewHtml = DOMPurify.sanitize(marked.parse(content) as string);
  $: void load(path);

  async function load(currentPath: string | null) {
    if (!currentPath) {
      message = t("not_created_document");
      content = "";
      original = "";
      specsMode = false;
      return;
    }

    if (isSpecsDirectoryPath(currentPath)) {
      specsMode = true;
      specsDir = currentPath;
      await refreshSpecsDocs();
      content = "";
      original = "";
      return;
    }
    specsMode = false;
    specsDir = "";
    specsDocs = [];
    selectedSpecPath = null;

    try {
      const fileContent = await invoke<string>("read_file", { path: currentPath });
      content = fileContent;
      original = fileContent;
      message = "";
    } catch (error) {
      content = "";
      original = "";
      const raw = String(error);
      if (raw.includes("Is a directory") || raw.includes("os error 21")) {
        message = t("no_documents");
      } else {
        message = raw;
      }
    }
  }

  function isSpecsDirectoryPath(currentPath: string) {
    return /\/specs\/?$/.test(currentPath);
  }

  async function refreshSpecsDocs() {
    if (!specsDir) return;
    try {
      const docs = await invoke<SpecDocument[]>("list_spec_documents", { specsDir });
      specsDocs = docs;
      if (docs.length === 0) {
        selectedSpecPath = null;
        message = t("no_documents");
        content = "";
        original = "";
      } else {
        const preferred = selectedSpecPath && docs.some((doc) => doc.path === selectedSpecPath) ? selectedSpecPath : docs[0].path;
        await openSpecDocument(preferred);
      }
    } catch (error) {
      message = String(error);
      specsDocs = [];
      selectedSpecPath = null;
      content = "";
      original = "";
    }
  }

  async function openSpecDocument(docPath: string) {
    selectedSpecPath = docPath;
    try {
      const fileContent = await invoke<string>("read_file", { path: docPath });
      content = fileContent;
      original = fileContent;
      message = "";
    } catch (error) {
      content = "";
      original = "";
      message = String(error);
    }
  }

  function normalizeCapability(value: string) {
    const raw = value.trim().toLowerCase();
    const slug = raw
      .replace(/[^a-z0-9]+/g, "-")
      .replace(/^-+/, "")
      .replace(/-+$/, "");
    return slug;
  }

  async function createSpec() {
    const capability = normalizeCapability(newCapability);
    if (!capability) {
      createError = t("invalid_capability");
      return;
    }
    creatingSpec = true;
    createError = "";
    try {
      const createdPath = await invoke<string>("create_spec_document", { specsDir, capability });
      newCapability = "";
      await refreshSpecsDocs();
      await openSpecDocument(createdPath);
    } catch (error) {
      createError = String(error);
    } finally {
      creatingSpec = false;
    }
  }

  async function save() {
    const target = specsMode ? selectedSpecPath : path;
    if (!target) return;
    await invoke("write_file", { path: target, content });
    original = content;
    message = t("saved");
    if (specsMode) {
      await refreshSpecsDocs();
    }
  }

  function cancel() {
    content = original;
    message = t("discarded_changes");
  }
</script>

<div class="editor">
  <div class="toolbar">
    <button class="primary" on:click={save} disabled={!path} aria-label={t("save")} title={t("save")}><Save size={14} /> {t("save")}</button>
    <button on:click={cancel} aria-label={t("cancel")} title={t("cancel")}><Undo2 size={14} /> {t("cancel")}</button>
    <button on:click={onClose} aria-label={t("back")} title={t("back")}><ArrowLeft size={14} /> {t("back")}</button>
    <span>{message}</span>
  </div>

  {#if !path}
    <p class="empty">{t("not_created_document")}</p>
  {:else if specsMode}
    <div class="specs-mode">
      <aside>
        <h4>{t("specifications")}</h4>
        {#if specsDocs.length === 0}
          <p class="empty">{t("no_documents")}</p>
        {:else}
          <ul>
            {#each specsDocs as doc}
              <li>
                <button
                  class:active={doc.path === selectedSpecPath}
                  on:click={() => void openSpecDocument(doc.path)}
                  aria-label={`${t("open")} ${doc.name}`}
                  title={`${t("open")} ${doc.name}`}
                >
                  {doc.name}
                </button>
              </li>
            {/each}
          </ul>
        {/if}

        <div class="create-block">
          <label for="new-capability">{t("new_capability")}</label>
          <input id="new-capability" type="text" bind:value={newCapability} placeholder="mi-capability" />
          <button class="primary" on:click={createSpec} disabled={creatingSpec} aria-label={t("create_spec")} title={t("create_spec")}>
            <Plus size={14} /> {t("create_spec")}
          </button>
          {#if createError}<p class="empty">{createError}</p>{/if}
        </div>
      </aside>

      <section>
        {#if selectedSpecPath}
          <div class="grid">
            <textarea bind:value={content}></textarea>
            <article>{@html previewHtml}</article>
          </div>
        {:else}
          <p class="empty">{t("no_documents")}</p>
        {/if}
      </section>
    </div>
  {:else}
    <div class="grid">
      <textarea bind:value={content}></textarea>
      <article>{@html previewHtml}</article>
    </div>
  {/if}
</div>

<style>
  .editor { display: flex; flex-direction: column; gap: 10px; color: var(--text-primary); min-width: 0; }
  .toolbar { display: flex; gap: 6px; align-items: center; flex-wrap: wrap; color: var(--text-secondary); font-size: var(--font-size-small); }
  .toolbar button { display: inline-flex; align-items: center; gap: 5px; }
  .grid { display: grid; grid-template-columns: minmax(0,1fr) minmax(0,1fr); gap: 10px; min-height: 420px; min-width: 0; }
  .specs-mode { display: grid; grid-template-columns: minmax(220px, 280px) minmax(0, 1fr); gap: 10px; min-height: 420px; min-width: 0; }
  aside { border: 1px solid var(--border-default); border-radius: 8px; padding: 8px; background: var(--bg-secondary); display: grid; gap: 8px; align-content: start; }
  h4 { margin: 0; font-size: var(--font-size-base); }
  ul { list-style: none; margin: 0; padding: 0; display: grid; gap: 6px; }
  li button { width: 100%; text-align: left; }
  li button.active { border-color: var(--accent-color); }
  .create-block { display: grid; gap: 6px; border-top: 1px solid var(--border-default); padding-top: 8px; }
  .create-block label { font-size: var(--font-size-small); color: var(--text-secondary); }
  .create-block button { display: inline-flex; align-items: center; gap: 6px; justify-content: center; }
  section { min-width: 0; }
  textarea { width: 100%; min-height: 420px; font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, Liberation Mono, Courier New, monospace; background: var(--bg-secondary); color: var(--text-primary); border: 1px solid var(--border-default); }
  article { border: 1px solid var(--border-default); background: var(--bg-secondary); padding: 10px; overflow: auto; color: var(--text-primary); font-size: var(--font-size-small); }
  .empty { color: var(--text-secondary); font-size: var(--font-size-small); }
  @media (max-width: 900px) { .grid, .specs-mode { grid-template-columns: 1fr; } }
</style>
