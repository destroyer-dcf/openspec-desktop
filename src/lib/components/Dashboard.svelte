<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Archive, Bug, CheckCircle, Circle, Copy, Eye, FolderPlus, GitBranch, Lightbulb, ListChecks, Pencil, Trash2, XCircle } from "lucide-svelte";
  import type { UiKey } from "$lib/i18n";
  import ProjectDescriptionPanel from "$lib/components/ProjectDescriptionPanel.svelte";
  import ActiveChangeModal from "$lib/components/ActiveChangeModal.svelte";
  import ArchivedChangeModal from "$lib/components/ArchivedChangeModal.svelte";
  import ProposalModal from "$lib/components/ProposalModal.svelte";
  import type { Artifact, Change, ProjectState, Proposal, ProposalDetail, ProposalList, ProposalType } from "$lib/types";
  import type { CardColorChoice, CardColorPrefs } from "$lib/ui-prefs";

  export let state: ProjectState | null;
  export let onSelectChange: (change: Change) => void;
  export let cardColors: CardColorPrefs;
  export let t: (key: UiKey) => string;
  export let language: "es" | "en" | "fr" | "de" | "pt" = "es";
  $: void onSelectChange;

  function iconForArtifact(artifact: Artifact) {
    if (artifact.present) return CheckCircle;
    return Circle;
  }

  $: totalTasks = state?.active_changes.reduce((acc, c) => acc + (c.tasks?.total ?? 0), 0) ?? 0;
  $: completedTasks = state?.active_changes.reduce((acc, c) => acc + (c.tasks?.complete ?? 0), 0) ?? 0;
  $: percent = totalTasks === 0 ? 0 : (completedTasks / totalTasks) * 100;
  let selectedActive: Change | null = null;
  let selectedArchived: Change | null = null;
  let activeProposals: Proposal[] = [];
  let archivedProposals: Proposal[] = [];
  let filterType: "all" | "feature" | "bug" = "all";
  let filterStatus: "active" | "archived" = "active";
  let proposalMessage = "";
  let copyModalOpen = false;
  let copyModalText = "";
  let deleteModalOpen = false;
  let deleteProposalPath = "";
  let deleteProposalName = "";
  let modalOpen = false;
  let editingPath: string | null = null;
  let modalName = "";
  let modalType: ProposalType = "feature";
  let modalContent = "";
  let modalSaving = false;
  let modalError = "";

  $: if (state?.root_path) {
    void loadProposals();
  }
  $: currentProposals = (filterStatus === "active" ? activeProposals : archivedProposals).filter((proposal) => {
    if (filterType === "all") return true;
    return proposal.proposal_type === filterType;
  });

  function formatArchivedDate(raw: string | null) {
    if (!raw) return t("version_unavailable");
    const millis = Number(raw) * 1000;
    if (!Number.isFinite(millis)) return t("version_unavailable");
    return new Date(millis).toLocaleDateString(language);
  }

  function formatProposalDate(raw: string) {
    const seconds = Number(raw);
    if (!Number.isFinite(seconds)) return raw || t("version_unavailable");
    return new Date(seconds * 1000).toLocaleDateString(language);
  }

  function taskPercent(change: Change) {
    const total = Number(change.tasks?.total ?? 0);
    const done = Number(change.tasks?.complete ?? 0);
    return total === 0 ? 0 : (done / total) * 100;
  }

  function toneClass(color: CardColorChoice) {
    return color === "none" ? "" : `tone-card tone-${color}`;
  }

  function activeChangeColor(change: Change) {
    const done = Number(change.tasks?.complete ?? 0);
    const total = Number(change.tasks?.total ?? 0);
    // Si OpenSpec reporta Pending/Blocked, se considera pendiente visualmente.
    if (change.status === "Pending" || change.status === "Blocked") {
      return cardColors.activePending;
    }
    // Solo completado en Ready + 100% tareas.
    const isDone = change.status === "Ready" && total > 0 && done >= total;
    return isDone ? cardColors.activeDone : cardColors.activePending;
  }

  function activeDocuments(change: Change) {
    return change.artifacts
      .filter((artifact) => {
        if (!artifact.path) return false;
        if (artifact.name === "specs") return true;
        return artifact.present;
      })
      .map((artifact) => ({ name: artifact.name, path: artifact.path as string }));
  }

  function isChangeComplete(change: Change) {
    const total = Number(change.tasks?.total ?? 0);
    const done = Number(change.tasks?.complete ?? 0);
    // Ready implica cambio cerrado funcionalmente aunque no tenga tasks cuantificables.
    if (change.status === "Ready" && total === 0) return true;
    return total > 0 && done >= total;
  }

  async function loadProposals() {
    if (!state) return;
    try {
      const list = await invoke<ProposalList>("list_proposals");
      activeProposals = list.active;
      archivedProposals = list.archived;
    } catch {
      activeProposals = [];
      archivedProposals = [];
    }
  }

  function openNewProposal() {
    editingPath = null;
    modalName = "";
    modalType = "feature";
    modalContent = "";
    modalError = "";
    modalOpen = true;
  }

  async function openEditProposal(path: string) {
    modalError = "";
    try {
      const detail = await invoke<ProposalDetail>("get_proposal", { path });
      editingPath = detail.proposal.path;
      modalName = detail.proposal.name;
      modalType = detail.proposal.proposal_type;
      modalContent = detail.content;
      modalOpen = true;
    } catch (error) {
      modalError = String(error);
    }
  }

  async function saveProposal(payload: { name: string; proposalType: ProposalType; content: string }) {
    modalSaving = true;
    modalError = "";
    try {
      await invoke("save_proposal", {
        input: {
          path: editingPath,
          name: payload.name,
          proposal_type: payload.proposalType,
          content: payload.content,
        },
      });
      modalOpen = false;
      await loadProposals();
    } catch (error) {
      modalError = String(error);
    } finally {
      modalSaving = false;
    }
  }

  async function archiveProposal(path: string) {
    await invoke("archive_proposals", { paths: [path] });
    await loadProposals();
  }

  function askDeleteProposal(path: string, name: string) {
    deleteProposalPath = path;
    deleteProposalName = name;
    deleteModalOpen = true;
  }

  async function confirmDeleteProposal() {
    if (!deleteProposalPath) return;
    await invoke("delete_proposals", { paths: [deleteProposalPath] });
    await loadProposals();
    proposalMessage = `Propuesta eliminada: ${deleteProposalName}`;
    deleteModalOpen = false;
    deleteProposalPath = "";
    deleteProposalName = "";
  }

  function proposalSummary(proposal: Proposal) {
    const text = (proposal.summary ?? "").trim();
    return text.length > 0 ? text : t("no_content");
  }

  async function copyProposalMarkdown(path: string) {
    proposalMessage = "";
    try {
      const detail = await invoke<ProposalDetail>("get_proposal", { path });
      const markdown = detail.content?.trim() ?? "";
      await copyText(markdown);
      copyModalText = `Copiado: ${detail.proposal.name}`;
      copyModalOpen = true;
    } catch (error) {
      copyModalText = `No se pudo copiar: ${String(error)}`;
      copyModalOpen = true;
    }
  }

  async function copyText(text: string) {
    try {
      await invoke("copy_to_clipboard", { text });
      return;
    } catch {
      // Fallback to browser APIs if native clipboard command fails.
    }

    if (navigator.clipboard?.writeText) {
      try {
        await navigator.clipboard.writeText(text);
        return;
      } catch {
        // Fallback for contexts where Clipboard API is blocked by user agent.
      }
    }

    const textarea = document.createElement("textarea");
    textarea.value = text;
    textarea.setAttribute("readonly", "true");
    textarea.style.position = "fixed";
    textarea.style.top = "-1000px";
    textarea.style.left = "-1000px";
    document.body.appendChild(textarea);
    textarea.focus();
    textarea.select();

    const ok = document.execCommand("copy");
    document.body.removeChild(textarea);
    if (!ok) {
      throw new Error("Copiado bloqueado por el navegador");
    }
  }

  async function copyChangeName(name: string) {
    try {
      await copyText(name);
      copyModalText = `Copiado: ${name}`;
      copyModalOpen = true;
    } catch (error) {
      copyModalText = `No se pudo copiar: ${String(error)}`;
      copyModalOpen = true;
    }
  }

  function onTasksToggled(changeName: string, content: string) {
    const total = (content.match(/^- \[[ xX]\] /gm) ?? []).length;
    const complete = (content.match(/^- \[[xX]\] /gm) ?? []).length;
    if (!state) return;
    state = {
      ...state,
      active_changes: state.active_changes.map((change) =>
        change.name === changeName ? { ...change, tasks: { complete, total } } : change
      ),
    };
  }
</script>

{#if !state}
  <p>Abre carpeta para cargar proyecto OpenSpec.</p>
{:else}
  <ProjectDescriptionPanel
    contexto={state?.config?.contexto}
    schema={state?.config?.schema}
    schemaLabel={t("schema")}
    unavailableLabel={t("version_unavailable")}
    {percent}
    {completedTasks}
    {totalTasks}
    activeChanges={state?.active_changes.length ?? 0}
    archivedChanges={state?.archived_changes.length ?? 0}
  />
  <section class="grid">
    <div class="panel proposals-panel">
      <h2><FolderPlus size={16} /> {t("proposals")}</h2>
      <div class="proposal-actions">
        <select bind:value={filterType} aria-label={t("filter_by_type")} title={t("filter_by_type")}>
          <option value="all">{t("type_all")}</option>
          <option value="feature">{t("type_feature")}</option>
          <option value="bug">{t("type_bug")}</option>
        </select>
        <select bind:value={filterStatus} aria-label={t("filter_by_status")} title={t("filter_by_status")}>
          <option value="active">{t("status_active")}</option>
          <option value="archived">{t("status_archived")}</option>
        </select>
        <button class="primary icon-only" on:click={openNewProposal} aria-label={t("add_proposal")} title={t("add_proposal")}>
          <FolderPlus size={14} />
        </button>
      </div>
      {#if currentProposals.length === 0}
        <p>{filterStatus === "active" ? t("no_active_proposals") : t("no_archived_proposals")}</p>
      {:else}
        <ul class="proposals-grid">
          {#each currentProposals as proposal}
            {@const proposalTone = proposal.proposal_type === "bug" ? cardColors.proposalBug : cardColors.proposalFeature}
            <li class={toneClass(proposalTone)}>
              <div class="proposal-title">
                {#if proposal.proposal_type === "bug"}
                  <span class="proposal-type bug"><Bug size={14} aria-label={t("bug")} /></span>
                {:else}
                  <span class="proposal-type feature"><Lightbulb size={14} aria-label={t("feature")} /></span>
                {/if}
                <strong>{proposal.name}</strong>
              </div>
              <p class="proposal-summary" title={proposalSummary(proposal)}>{proposalSummary(proposal)}</p>
              <span>{proposal.proposal_type === "bug" ? t("bug") : t("feature")} · {formatProposalDate(proposal.created_at)}</span>
              <div class="proposal-row-actions">
                {#if filterStatus === "active"}
                  <button
                    class="icon-only"
                    on:click={() => void openEditProposal(proposal.path)}
                    aria-label={`${t("modify")} ${proposal.name}`}
                    title={`${t("modify")} ${proposal.name}`}
                  >
                    <Pencil size={14} />
                  </button>
                {/if}
                <button
                  class="icon-only"
                  on:click={() => void copyProposalMarkdown(proposal.path)}
                  aria-label={`${t("copy_markdown")} ${proposal.name}`}
                  title={`${t("copy_markdown")} ${proposal.name}`}
                >
                  <Copy size={14} />
                </button>
                {#if filterStatus === "active"}
                  <button
                    class="icon-only"
                    on:click={() => void archiveProposal(proposal.path)}
                    aria-label={`${t("archive")} ${proposal.name}`}
                    title={`${t("archive")} ${proposal.name}`}
                  >
                    <Archive size={14} />
                  </button>
                  <button
                    class="icon-only danger"
                    on:click={() => askDeleteProposal(proposal.path, proposal.name)}
                    aria-label={`${t("delete")} ${proposal.name}`}
                    title={`${t("delete")} ${proposal.name}`}
                  >
                    <Trash2 size={14} />
                  </button>
                {/if}
              </div>
            </li>
          {/each}
        </ul>
      {/if}
    </div>

    <div class="panel active-panel">
      <h2><GitBranch size={16} /> {t("active_changes")}</h2>
      {#if state.active_changes.length === 0}
        <p>{t("no_active_changes")}</p>
      {:else}
        <ul class="changes">
          {#each state.active_changes as change}
            {@const activeTone = activeChangeColor(change)}
            <li class={toneClass(activeTone)}>
              <div class="change-head">
                <strong>{change.name}</strong>
              </div>
              <p class="why-summary" title={change.why_summary}>{change.why_summary || t("no_summary")}</p>
              <div class="artifacts">
                {#each change.artifacts as artifact}
                  {@const Icon = iconForArtifact(artifact)}
                  <span class="artifact" title={artifact.name}>
                    <Icon size={13} /> {artifact.name}
                  </span>
                {/each}
                <span class="artifact" title={`tasks: ${change.tasks?.complete ?? 0}/${change.tasks?.total ?? 0}`}>
                  <ListChecks size={13} /> {change.tasks?.complete ?? 0}/{change.tasks?.total ?? 0}
                </span>
                {#if change.status === "Blocked"}
                  <span class="artifact blocked"><XCircle size={13} /> {t("blocked")}</span>
                {/if}
              </div>
              <div class="change-head-actions">
                <strong class="change-percent-text">{Math.round(taskPercent(change))}%</strong>
                <div class="change-buttons">
                  <button class="icon-only" on:click={() => void copyChangeName(change.name)} aria-label={`${t("copy")} ${change.name}`} title={`${t("copy")} ${change.name}`}>
                    <Copy size={14} />
                  </button>
                  <button class="icon-only" on:click={() => (selectedActive = change)} aria-label={`${t("view")} ${change.name}`} title={`${t("view")} ${change.name}`}>
                    <Eye size={14} />
                  </button>
                </div>
              </div>
            </li>
          {/each}
        </ul>
      {/if}
    </div>

    <div class="panel archived-panel">
      <h2><Archive size={16} /> {t("archived_changes")}</h2>
      {#if state.archived_changes.length === 0}
        <p>{t("no_archived_changes")}</p>
      {:else}
        <ul class="archived-grid">
          {#each state.archived_changes as change}
            <li class={toneClass(cardColors.archived)}>
              <strong>{change.name}</strong>
              <p class="why-summary" title={change.why_summary}>{change.why_summary || t("no_summary")}</p>
              <span>{t("apply_date")}: {formatArchivedDate(change.archived_at)}</span>
              <button class="icon-only" on:click={() => (selectedArchived = change)} aria-label={`${t("view")} ${change.name}`} title={`${t("view")} ${change.name}`}>
                <Eye size={14} />
              </button>
            </li>
          {/each}
        </ul>
      {/if}
    </div>
  </section>

  {#if selectedArchived}
    <ArchivedChangeModal
      changeName={selectedArchived.name}
      documents={selectedArchived.archived_documents}
      {t}
      onClose={() => (selectedArchived = null)}
    />
  {/if}

  {#if selectedActive}
    <ActiveChangeModal
      changeName={selectedActive.name}
      documents={activeDocuments(selectedActive)}
      canModify={!isChangeComplete(selectedActive)}
      {t}
      onClose={() => (selectedActive = null)}
      onTaskToggled={(content) => onTasksToggled(selectedActive?.name ?? "", content)}
    />
  {/if}

  <ProposalModal
    open={modalOpen}
    title={t("proposals")}
    name={modalName}
    proposalType={modalType}
    content={modalContent}
    saving={modalSaving}
    error={modalError}
    onClose={() => (modalOpen = false)}
    onSave={saveProposal}
  />

  {#if copyModalOpen}
    <div class="modal" role="dialog" aria-modal="true" aria-label={t("clipboard_result")}>
      <div class="sheet small">
        <h3>{t("clipboard_result")}</h3>
        <p>{copyModalText}</p>
        <div class="actions">
          <button class="primary" on:click={() => (copyModalOpen = false)}>{t("ok")}</button>
        </div>
      </div>
    </div>
  {/if}

  {#if deleteModalOpen}
    <div class="modal" role="dialog" aria-modal="true" aria-label={t("confirm_delete_title")}>
      <div class="sheet small">
        <h3>{t("confirm_delete_title")}</h3>
        <p>{t("delete_proposal_confirm")} "{deleteProposalName}"? {t("delete_proposal_irreversible")}</p>
        <div class="actions">
          <button on:click={() => { deleteModalOpen = false; deleteProposalPath = ""; deleteProposalName = ""; }}>{t("cancel")}</button>
          <button class="primary danger" on:click={confirmDeleteProposal}>{t("delete")}</button>
        </div>
      </div>
    </div>
  {/if}
{/if}

<style>
  .grid { display: grid; grid-template-columns: minmax(300px,1fr) minmax(340px,1.2fr) minmax(300px,1fr); gap: 12px; min-width: 0; align-items: start; }
  .panel { border: 1px solid var(--border-default); border-radius: 8px; padding: 10px; background: var(--bg-secondary); color: var(--text-primary); min-width: 0; overflow: hidden; }
  h2 { display: flex; align-items: center; gap: 6px; margin: 0 0 8px 0; font-size: var(--font-size-base); }
  h3 { display: flex; align-items: center; gap: 6px; margin-top: 12px; font-size: var(--font-size-base); }
  .changes { list-style: none; margin: 0; padding: 0; display: grid; gap: 8px; }
  .changes li { border: 1px solid var(--border-default); border-radius: 8px; padding: 8px; display: grid; gap: 6px; background: var(--bg-primary); position: relative; }
  .change-head { display: flex; align-items: center; justify-content: space-between; gap: 8px; min-width: 0; }
  .change-head strong { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .change-head-actions { display: flex; gap: 10px; align-items: center; justify-content: flex-start; min-width: 0; }
  .change-buttons { display: inline-flex; gap: 6px; flex-wrap: wrap; }
  .change-percent-text {
    font-size: calc(var(--font-size-large) + 2px);
    font-weight: 800;
    line-height: 1;
    white-space: nowrap;
  }
  .artifacts { display: flex; flex-wrap: wrap; gap: 6px; }
  .artifact { display: inline-flex; align-items: center; gap: 4px; font-size: var(--font-size-small); color: var(--text-secondary); }
  .blocked { color: var(--danger-color); }
  .archived-grid { list-style: none; margin: 0; padding: 0; display: grid; grid-template-columns: 1fr; gap: 8px; }
  .archived-grid li { border: 1px solid var(--border-default); border-radius: 8px; background: var(--bg-primary); padding: 8px; display: grid; gap: 6px; }
  .archived-grid li strong { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .archived-grid li span { color: var(--text-secondary); font-size: var(--font-size-small); }
  .archived-grid li button { justify-self: start; }
  .proposal-actions { display: flex; gap: 8px; margin: 8px 0; flex-wrap: wrap; align-items: center; }
  .proposal-actions select { padding: 4px 6px; font-size: var(--font-size-small); min-width: 140px; }
  .icon-only { width: 30px; height: 30px; display: inline-flex; align-items: center; justify-content: center; padding: 0; }
  .proposals-grid { list-style: none; margin: 0; padding: 0; display: grid; grid-template-columns: 1fr; gap: 8px; }
  .proposals-grid li { border: 1px solid var(--border-default); border-radius: 8px; background: var(--bg-primary); padding: 8px; display: grid; gap: 6px; }
  .changes li.tone-card,
  .archived-grid li.tone-card,
  .proposals-grid li.tone-card {
    background: var(--tone-bg);
    border-color: var(--tone-border);
    color: var(--tone-text);
  }
  .changes li.tone-card .artifact,
  .changes li.tone-card .change-head strong,
  .changes li.tone-card .blocked,
  .archived-grid li.tone-card strong,
  .archived-grid li.tone-card span,
  .proposals-grid li.tone-card strong,
  .proposals-grid li.tone-card span {
    color: var(--tone-text);
  }
  .changes li.tone-card .icon-only,
  .archived-grid li.tone-card .icon-only,
  .proposals-grid li.tone-card .icon-only {
    background: rgba(255, 255, 255, 0.18);
    border-color: var(--tone-border);
    color: var(--tone-text);
  }
  .changes li.tone-card .icon-only:hover,
  .archived-grid li.tone-card .icon-only:hover,
  .proposals-grid li.tone-card .icon-only:hover {
    background: rgba(255, 255, 255, 0.26);
  }
  .tone-blue {
    --tone-bg: rgba(9, 105, 218, 0.16);
    --tone-border: #0969da;
    --tone-text: var(--text-primary);
  }
  .tone-green {
    --tone-bg: rgba(45, 164, 78, 0.16);
    --tone-border: #2da44e;
    --tone-text: var(--text-primary);
  }
  .tone-red {
    --tone-bg: rgba(207, 34, 46, 0.14);
    --tone-border: #cf222e;
    --tone-text: var(--text-primary);
  }
  .tone-yellow {
    --tone-bg: rgba(191, 135, 0, 0.18);
    --tone-border: #bf8700;
    --tone-text: var(--text-primary);
  }
  .tone-gray {
    --tone-bg: rgba(110, 119, 129, 0.14);
    --tone-border: #6e7781;
    --tone-text: var(--text-primary);
  }
  .tone-orange {
    --tone-bg: rgba(188, 76, 0, 0.16);
    --tone-border: #bc4c00;
    --tone-text: var(--text-primary);
  }
  .proposal-title { display: inline-flex; align-items: center; gap: 6px; min-width: 0; }
  .proposal-title strong { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .proposal-type.bug { color: var(--danger-color); flex-shrink: 0; }
  .proposal-type.feature { color: #2da44e; flex-shrink: 0; }
  .proposals-grid li span { color: var(--text-secondary); font-size: var(--font-size-small); }
  .proposal-summary {
    margin: 0;
    color: var(--text-secondary);
    font-size: var(--font-size-small);
    line-height: 1.35;
    display: -webkit-box;
    line-clamp: 2;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }
  .why-summary {
    margin: 0;
    color: var(--text-secondary);
    font-size: var(--font-size-small);
    line-height: 1.35;
    display: -webkit-box;
    line-clamp: 2;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }
  .proposal-row-actions { display: flex; gap: 6px; }
  .panel p { color: var(--text-secondary); font-size: var(--font-size-small); }
  .modal { position: fixed; inset: 0; background: var(--overlay-backdrop); display: grid; place-items: center; z-index: 70; }
  .sheet.small { width: min(440px, calc(100vw - 24px)); border: 1px solid var(--border-default); border-radius: 10px; background: var(--bg-primary); padding: 12px; display: grid; gap: 10px; }
  .sheet.small p { margin: 0; color: var(--text-secondary); font-size: var(--font-size-small); }
  .actions { display: flex; justify-content: flex-end; gap: 12px; }
  .danger { background: var(--danger-color); border-color: var(--danger-color); color: #ffffff; }
  @media (max-width: 1200px) {
    .grid { grid-template-columns: minmax(280px,1fr) minmax(320px,1.2fr) minmax(280px,1fr); }
  }
</style>
