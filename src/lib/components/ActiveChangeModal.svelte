<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { marked } from "marked";
  import DOMPurify from "dompurify";
  import { Eye, Pencil, Plus, Save, Undo2, X } from "lucide-svelte";
  import type { UiKey } from "$lib/i18n";

  export let changeName = "";
  export let documents: { name: string; path: string }[] = [];
  export let canModify = true;
  export let t: (key: UiKey) => string;
  export let onClose: () => void;
  export let onTaskToggled: (content: string) => void = () => {};
  type SpecDocument = { name: string; path: string };

  let selectedPath: string | null = null;
  let selectedName = "";
  let content = "";
  let original = "";
  let message = "";
  let loading = false;
  let editing = false;
  let specsMode = false;
  let specsDir = "";
  let specsDocs: SpecDocument[] = [];
  let selectedSpecPath: string | null = null;
  let creatingSpec = false;
  let newCapability = "";
  let createError = "";

  $: if (!selectedPath && documents.length > 0) {
    void selectDocument(documents[0]);
  }

  $: previewHtml = DOMPurify.sanitize(marked.parse(content) as string);
  $: canToggleTasks = selectedName.toLowerCase().includes("tasks");

  async function selectDocument(doc: { name: string; path: string }) {
    selectedPath = doc.path;
    selectedName = doc.name;
    editing = false;
    loading = true;
    message = "";
    if (isSpecsDirPath(doc.path)) {
      specsMode = true;
      specsDir = doc.path;
      selectedSpecPath = null;
      await refreshSpecsDocs();
      loading = false;
      return;
    }
    specsMode = false;
    specsDir = "";
    specsDocs = [];
    selectedSpecPath = null;
    try {
      const fileContent = await invoke<string>("read_file", { path: doc.path });
      content = fileContent;
      original = fileContent;
    } catch (error) {
      content = "";
      original = "";
      message = String(error);
    } finally {
      loading = false;
    }
  }

  function isSpecsDirPath(path: string) {
    return /\/specs\/?$/.test(path);
  }

  async function refreshSpecsDocs() {
    if (!specsDir) return;
    try {
      const docs = await invoke<SpecDocument[]>("list_spec_documents", { specsDir });
      specsDocs = docs;
      if (docs.length === 0) {
        message = "No existen ficheros de especificaciones";
        content = "";
        original = "";
        selectedSpecPath = null;
      } else {
        const preferred = selectedSpecPath && docs.some((doc) => doc.path === selectedSpecPath) ? selectedSpecPath : docs[0].path;
        await openSpecDocument(preferred);
      }
    } catch (error) {
      message = String(error);
      content = "";
      original = "";
      selectedSpecPath = null;
      specsDocs = [];
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
    return raw.replace(/[^a-z0-9]+/g, "-").replace(/^-+/, "").replace(/-+$/, "");
  }

  async function createSpec() {
    if (!canModify) return;
    const capability = normalizeCapability(newCapability);
    if (!capability) {
      createError = "Nombre de capability inválido";
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
    if (!canModify) return;
    const target = specsMode ? selectedSpecPath : selectedPath;
    if (!target) return;
    await invoke("write_file", { path: target, content });
    original = content;
    message = "Guardado";
    if (specsMode) {
      await refreshSpecsDocs();
    }
  }

  function cancel() {
    content = original;
    editing = false;
    message = "Cambios descartados";
  }

  async function toggleTaskAtIndex(index: number, checked: boolean) {
    const lines = content.split("\n");
    let found = -1;
    for (let i = 0; i < lines.length; i += 1) {
      if (/^- \[[ xX]\] /.test(lines[i].trim())) {
        found += 1;
        if (found === index) {
          lines[i] = lines[i].replace(/^- \[[ xX]\]/, checked ? "- [x]" : "- [ ]");
          break;
        }
      }
    }
    if (found < index) return;
    const next = lines.join("\n");
    content = next;
    original = next;
    if (selectedPath) {
      await invoke("write_file", { path: selectedPath, content: next });
      onTaskToggled(next);
      message = "Tarea actualizada";
    }
  }

  async function onPreviewClick(event: MouseEvent) {
    if (!canToggleTasks || editing || !canModify) return;
    const target = event.target as HTMLElement | null;
    if (!target || target.tagName !== "INPUT") return;
    const input = target as HTMLInputElement;
    if (input.type !== "checkbox") return;
    event.preventDefault();

    const root = input.closest("article");
    if (!root) return;
    const all = Array.from(root.querySelectorAll<HTMLInputElement>('input[type="checkbox"]'));
    const idx = all.indexOf(input);
    if (idx < 0) return;
    await toggleTaskAtIndex(idx, !input.checked);
  }

  async function onPreviewKeydown(event: KeyboardEvent) {
    if (event.key !== "Enter" && event.key !== " ") return;
    await onPreviewClick(event as unknown as MouseEvent);
  }
</script>

<div class="modal" role="dialog" aria-modal="true" aria-label={`${t("view")} ${changeName}`}>
  <div class="sheet">
    <header>
      <h3>{t("active_change")}: {changeName}</h3>
      <div class="actions">
        <button class:active={!editing} on:click={() => (editing = false)} aria-label={t("view")} title={t("preview")}>
          <Eye size={14} />
        </button>
        <button
          class:active={editing}
          on:click={() => (editing = true)}
          disabled={!canModify || (specsMode && !selectedSpecPath)}
          aria-label={t("edit")}
          title={!canModify ? t("read_only_complete") : t("edit")}
        >
          <Pencil size={14} />
        </button>
        <button
          on:click={save}
          disabled={!canModify || (!selectedPath && !selectedSpecPath)}
          aria-label={t("save")}
          title={!canModify ? t("read_only_complete") : t("save")}
        >
          <Save size={14} />
        </button>
        <button on:click={cancel} aria-label={t("cancel")} title={t("cancel")}>
          <Undo2 size={14} />
        </button>
        <button on:click={onClose} aria-label={t("close")} title={t("close")}><X size={14} /></button>
      </div>
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
                  aria-label={`${t("open")} ${doc.name}`}
                  title={`${t("open")} ${doc.name}`}
                >
                  {doc.name}
                </button>
              </li>
            {/each}
          </ul>
        {/if}
      </aside>

      <section>
        <h4>{selectedName || t("document")} {message ? `· ${message}` : ""}</h4>
        {#if loading}
          <p>{t("loading")}</p>
        {:else if specsMode}
          <div class="specs-mode">
            <aside class="specs-aside">
              <h4>Especificaciones</h4>
              {#if specsDocs.length === 0}
                <p>No existen ficheros de especificaciones</p>
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
              {#if canModify}
                <div class="create-block">
                  <label for="new-capability">Nueva capability</label>
                  <input id="new-capability" type="text" bind:value={newCapability} placeholder="mi-capability" />
                  <button class="primary" on:click={createSpec} disabled={creatingSpec} aria-label={t("create_spec")} title={t("create_spec")}>
                    <Plus size={14} /> {t("create_spec")}
                  </button>
                  {#if createError}<p>{createError}</p>{/if}
                </div>
              {/if}
            </aside>
            <div class="specs-content">
              {#if selectedSpecPath}
                {#if editing}
                  <textarea bind:value={content}></textarea>
                {:else}
                  <div class="preview-box" role="button" tabindex="0" on:click={onPreviewClick} on:keydown={onPreviewKeydown}>
                    <article>
                      {@html previewHtml}
                    </article>
                  </div>
                {/if}
              {:else}
                <p>No existen ficheros de especificaciones</p>
              {/if}
            </div>
          </div>
        {:else if editing}
          <textarea bind:value={content}></textarea>
        {:else}
          <div class="preview-box" role="button" tabindex="0" on:click={onPreviewClick} on:keydown={onPreviewKeydown}>
            <article>
              {@html previewHtml}
            </article>
          </div>
        {/if}
      </section>
    </div>
  </div>
</div>

<style>
  .modal { position: fixed; inset: 0; background: var(--overlay-backdrop); display: grid; place-items: center; z-index: 60; }
  .sheet { width: min(1000px, calc(100vw - 40px)); height: min(700px, calc(100vh - 40px)); overflow: hidden; border: 1px solid var(--border-default); border-radius: 10px; background: var(--bg-primary); display: grid; grid-template-rows: auto minmax(0, 1fr); }
  header { display: flex; align-items: center; justify-content: space-between; gap: 8px; padding: 10px; border-bottom: 1px solid var(--border-default); }
  .actions { display: flex; align-items: center; gap: 6px; }
  .actions button:disabled {
    opacity: 0.45;
    cursor: not-allowed;
  }
  .actions button.active { border-color: var(--accent-color); color: var(--accent-color); }
  h3, h4 { margin: 0; }
  .body { display: grid; grid-template-columns: minmax(220px, 280px) minmax(0, 1fr); min-height: 0; }
  aside { border-right: 1px solid var(--border-default); padding: 10px; min-width: 0; overflow: auto; }
  section { padding: 10px; min-height: 0; overflow: hidden; display: grid; grid-template-rows: auto minmax(0, 1fr); gap: 8px; }
  ul { list-style: none; margin: 8px 0 0 0; padding: 0; display: grid; gap: 6px; }
  li button { width: 100%; text-align: left; }
  li button.active { border-color: var(--accent-color); }
  textarea { width: 100%; min-height: 0; height: 100%; font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, Liberation Mono, Courier New, monospace; background: var(--bg-secondary); color: var(--text-primary); border: 1px solid var(--border-default); }
  article { border: 1px solid var(--border-default); border-radius: 8px; background: var(--bg-secondary); padding: 10px; font-size: var(--font-size-small); min-height: 0; overflow: auto; }
  .preview-box { min-height: 0; overflow: hidden; }
  .specs-mode { display: grid; grid-template-columns: minmax(220px, 280px) minmax(0, 1fr); gap: 10px; min-height: 0; }
  .specs-aside { border: 1px solid var(--border-default); border-radius: 8px; padding: 8px; min-height: 0; overflow: auto; }
  .specs-content { min-height: 0; overflow: hidden; display: grid; }
  .create-block { border-top: 1px solid var(--border-default); margin-top: 8px; padding-top: 8px; display: grid; gap: 6px; }
  .create-block label { color: var(--text-secondary); font-size: var(--font-size-small); }
  .create-block button { display: inline-flex; align-items: center; justify-content: center; gap: 6px; }
  article :global(h1), article :global(h2), article :global(h3), article :global(h4) { color: var(--text-primary); font-weight: 700; }
  p { color: var(--text-secondary); font-size: var(--font-size-small); }
</style>
