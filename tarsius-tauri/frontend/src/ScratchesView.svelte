<script>
  import { onMount } from "svelte";

  let scratches = [];
  let selectedScratch = null;
  let searchQuery = "";
  let filteredScratches = [];
  let editorContent = "";
  let isCreating = false;
  let newScratchTitle = "";
  let newScratchContent = "";
  let autosaveTimeout;

  let isTauri = typeof window !== "undefined" && window.__TAURI__;
  console.log("ScratchesView: Tauri environment detected:", isTauri);

  onMount(async () => {
    console.log("ScratchesView mounted");
    await loadScratches();
  });

  async function loadScratches() {
    try {
      if (isTauri) {
        const { invoke } = await import("@tauri-apps/api/tauri");
        console.log("Loading scratches from Tauri...");
        scratches = await invoke("list_scratches");
        console.log("Loaded scratches from Tauri:", scratches);
      } else {
        console.log("Using mock scratches data (not in Tauri environment)");
        scratches = [
          {
            id: "1",
            title: "Introduction to LaTeX",
            content:
              "LaTeX is a high-quality typesetting system; it includes features designed for the production of technical and scientific documentation.",
            tags: ["latex", "intro"],
            created_at: new Date().toISOString(),
            modified_at: new Date().toISOString(),
          },
          {
            id: "2",
            title: "Research Notes",
            content:
              "Key findings from the literature review on document compilation systems and their impact on academic writing workflows.",
            tags: ["research", "notes"],
            created_at: new Date().toISOString(),
            modified_at: new Date().toISOString(),
          },
          {
            id: "3",
            title: "Project Ideas",
            content:
              "Brainstorming session for potential improvements to the Tarsius application including better syntax highlighting and template management.",
            tags: ["ideas", "planning"],
            created_at: new Date().toISOString(),
            modified_at: new Date().toISOString(),
          },
        ];
      }
      filterScratches();
    } catch (e) {
      console.error("Error loading scratches:", e);
      scratches = [
        {
          id: "1",
          title: "Introduction to LaTeX",
          content: "LaTeX is a high-quality typesetting system...",
          tags: ["latex", "intro"],
          created_at: new Date().toISOString(),
          modified_at: new Date().toISOString(),
        },
        {
          id: "2",
          title: "Research Notes",
          content: "Key findings from the literature review...",
          tags: ["research", "notes"],
          created_at: new Date().toISOString(),
          modified_at: new Date().toISOString(),
        },
      ];
      filterScratches();
    }
  }

  function handleEditorChange() {
    if (autosaveTimeout) clearTimeout(autosaveTimeout);
    autosaveTimeout = setTimeout(updateScratch, 1000);
  }

  $: if (searchQuery !== undefined) {
    filterScratches();
  }

  function filterScratches() {
    if (!searchQuery) {
      filteredScratches = scratches;
    } else {
      const query = searchQuery.toLowerCase();
      filteredScratches = scratches.filter(
        (scratch) =>
          scratch.title.toLowerCase().includes(query) ||
          scratch.content.toLowerCase().includes(query) ||
          scratch.tags.some((tag) => tag.toLowerCase().includes(query)),
      );
    }
  }

  async function createScratch() {
    if (!newScratchTitle.trim()) return;
    try {
      if (isTauri) {
        const { invoke } = await import("@tauri-apps/api/tauri");
        await invoke("create_scratch", {
          title: newScratchTitle,
          content: newScratchContent,
          tags: [],
          source: null,
        });
        await loadScratches();
      } else {
        const newScratch = {
          id: Date.now().toString(),
          title: newScratchTitle,
          content: newScratchContent,
          tags: [],
          created_at: new Date().toISOString(),
          modified_at: new Date().toISOString(),
        };
        scratches = [...scratches, newScratch];
        filterScratches();
      }
    } catch (e) {
      console.error("Error creating scratch:", e);
    }

    newScratchTitle = "";
    newScratchContent = "";
    isCreating = false;
  }

  function selectScratch(scratch) {
    selectedScratch = scratch;
    editorContent = scratch.content;
  }

  async function updateScratch() {
    if (!selectedScratch) return;

    try {
      if (isTauri) {
        const { invoke } = await import("@tauri-apps/api/tauri");
        await invoke("update_scratch", {
          id: selectedScratch.id,
          title: selectedScratch.title,
          content: editorContent,
          tags: selectedScratch.tags,
          source: selectedScratch.source,
        });
      } else {
        scratches = scratches.map((s) =>
          s.id === selectedScratch.id
            ? {
                ...s,
                content: editorContent,
                modified_at: new Date().toISOString(),
              }
            : s,
        );
      }
    } catch (e) {
      console.error("Error updating scratch:", e);
    }
  }
</script>

<div class="scratches-view">
  <!-- Header with Search and Actions -->
  <div class="view-header">
    <div class="header-content">
      <h1 class="view-title">Scratches</h1>
      <p class="view-subtitle">
        {filteredScratches.length} note{filteredScratches.length !== 1
          ? "s"
          : ""}
      </p>
    </div>

    <div class="header-actions">
      <div class="search-container">
        <svg
          class="search-icon"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          <circle cx="11" cy="11" r="8" />
          <path d="m21 21-4.35-4.35" />
        </svg>
        <input
          type="text"
          bind:value={searchQuery}
          placeholder="Search scratches..."
          class="search-input"
        />
      </div>

      <button
        class="btn btn-primary"
        on:click={() => (isCreating = !isCreating)}
      >
        <svg
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          style="width: 16px; height: 16px;"
        >
          <path d="M12 5v14m-7-7h14" />
        </svg>
        {isCreating ? "Cancel" : "New Scratch"}
      </button>
    </div>
  </div>

  <!-- Creation Form -->
  {#if isCreating}
    <div class="create-form card animate-slide-up">
      <h3 class="form-title">Create New Scratch</h3>
      <input
        bind:value={newScratchTitle}
        placeholder="Title"
        class="input"
        style="margin-bottom: var(--space-md);"
      />
      <textarea
        bind:value={newScratchContent}
        placeholder="Content"
        rows="5"
        class="input"
        style="resize: vertical; font-family: var(--font-mono); margin-bottom: var(--space-md);"
      ></textarea>
      <div
        style="display: flex; gap: var(--space-sm); justify-content: flex-end;"
      >
        <button class="btn btn-secondary" on:click={() => (isCreating = false)}
          >Cancel</button
        >
        <button class="btn btn-primary" on:click={createScratch}>Create</button>
      </div>
    </div>
  {/if}

  <!-- Main Content -->
  <div class="main-content">
    <!-- Scratches List -->
    <div class="list-panel">
      <div class="scratches-grid">
        {#each filteredScratches as scratch}
          <div
            class="scratch-card card"
            class:selected={selectedScratch &&
              selectedScratch.id === scratch.id}
            on:click={() => selectScratch(scratch)}
          >
            <div class="scratch-card-header">
              <h3 class="scratch-card-title">{scratch.title}</h3>
              <span class="scratch-date"
                >{new Date(scratch.modified_at).toLocaleDateString()}</span
              >
            </div>
            <p class="scratch-card-preview">
              {scratch.content.substring(0, 120)}{scratch.content.length > 120
                ? "..."
                : ""}
            </p>
            <div class="scratch-card-footer">
              {#each scratch.tags as tag}
                <span class="tag">{tag}</span>
              {/each}
            </div>
          </div>
        {/each}
      </div>
    </div>

    <!-- Editor Panel -->
    <div class="editor-panel">
      {#if selectedScratch}
        <div class="editor-container">
          <div class="editor-header">
            <input
              bind:value={selectedScratch.title}
              on:input={handleEditorChange}
              class="editor-title-input"
              placeholder="Untitled"
            />
            <div class="editor-meta">
              <span class="meta-item">
                <svg
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                  style="width: 14px; height: 14px;"
                >
                  <path d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
                {new Date(selectedScratch.modified_at).toLocaleString()}
              </span>
            </div>
          </div>

          <textarea
            bind:value={editorContent}
            on:input={() => {
              selectedScratch.content = editorContent;
              handleEditorChange();
            }}
            class="editor-textarea"
            placeholder="Start writing..."
          ></textarea>

          <div class="editor-footer">
            <div class="tag-container">
              {#each selectedScratch.tags as tag}
                <span class="tag">{tag}</span>
              {/each}
            </div>
            <span class="autosave-indicator">
              <svg
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                style="width: 14px; height: 14px;"
              >
                <path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              Auto-saved
            </span>
          </div>
        </div>
      {:else}
        <div class="empty-state">
          <svg
            class="empty-icon"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          >
            <path
              d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"
            />
          </svg>
          <h3>No Scratch Selected</h3>
          <p>Select a scratch from the list to view and edit</p>
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .scratches-view {
    height: 100%;
    display: flex;
    flex-direction: column;
    background: var(--color-bg-primary);
  }

  /* ============================================
     HEADER
     ============================================ */

  .view-header {
    padding: var(--space-xl) var(--space-2xl);
    border-bottom: 1px solid var(--color-border-primary);
    background: var(--color-bg-secondary);
  }

  .header-content {
    margin-bottom: var(--space-lg);
  }

  .view-title {
    font-size: var(--font-size-3xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text-primary);
    margin-bottom: var(--space-xs);
  }

  .view-subtitle {
    font-size: var(--font-size-sm);
    color: var(--color-text-secondary);
  }

  .header-actions {
    display: flex;
    gap: var(--space-md);
    align-items: center;
  }

  .search-container {
    flex: 1;
    position: relative;
    max-width: 400px;
  }

  .search-icon {
    position: absolute;
    left: var(--space-md);
    top: 50%;
    transform: translateY(-50%);
    width: 18px;
    height: 18px;
    color: var(--color-text-muted);
    pointer-events: none;
  }

  .search-input {
    width: 100%;
    padding-left: calc(var(--space-md) * 3);
  }

  /* ============================================
     CREATE FORM
     ============================================ */

  .create-form {
    margin: var(--space-xl) var(--space-2xl);
  }

  .form-title {
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-primary);
    margin-bottom: var(--space-lg);
  }

  /* ============================================
     MAIN CONTENT
     ============================================ */

  .main-content {
    flex: 1;
    display: flex;
    overflow: hidden;
  }

  /* ============================================
     LIST PANEL
     ============================================ */

  .list-panel {
    width: 400px;
    border-right: 1px solid var(--color-border-primary);
    overflow-y: auto;
    background: var(--color-bg-secondary);
  }

  .scratches-grid {
    padding: var(--space-lg);
    display: flex;
    flex-direction: column;
    gap: var(--space-md);
  }

  .scratch-card {
    cursor: pointer;
    transition: all var(--transition-fast);
    animation: slideInLeft var(--transition-base);
  }

  .scratch-card.selected {
    border-color: var(--color-accent-primary);
    background: var(--color-bg-tertiary);
    box-shadow: var(--shadow-glow);
  }

  .scratch-card-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: var(--space-sm);
    gap: var(--space-md);
  }

  .scratch-card-title {
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-primary);
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .scratch-date {
    font-size: var(--font-size-xs);
    color: var(--color-text-tertiary);
    flex-shrink: 0;
  }

  .scratch-card-preview {
    font-size: var(--font-size-sm);
    color: var(--color-text-secondary);
    line-height: 1.5;
    margin-bottom: var(--space-md);
  }

  .scratch-card-footer {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-xs);
  }

  /* ============================================
     EDITOR PANEL
     ============================================ */

  .editor-panel {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .editor-container {
    flex: 1;
    display: flex;
    flex-direction: column;
    padding: var(--space-2xl);
  }

  .editor-header {
    margin-bottom: var(--space-xl);
  }

  .editor-title-input {
    width: 100%;
    font-size: var(--font-size-3xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text-primary);
    background: transparent;
    border: none;
    outline: none;
    padding: 0;
    margin-bottom: var(--space-md);
  }

  .editor-title-input::placeholder {
    color: var(--color-text-muted);
  }

  .editor-meta {
    display: flex;
    gap: var(--space-lg);
  }

  .meta-item {
    display: flex;
    align-items: center;
    gap: var(--space-xs);
    font-size: var(--font-size-xs);
    color: var(--color-text-tertiary);
  }

  .editor-textarea {
    flex: 1;
    width: 100%;
    padding: var(--space-lg);
    font-family: var(--font-mono);
    font-size: var(--font-size-base);
    line-height: 1.8;
    color: var(--color-text-primary);
    background: var(--color-bg-tertiary);
    border: 1px solid var(--color-border-primary);
    border-radius: var(--radius-lg);
    resize: none;
    outline: none;
    transition: all var(--transition-fast);
  }

  .editor-textarea:focus {
    border-color: var(--color-accent-primary);
    box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
  }

  .editor-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: var(--space-lg);
    padding-top: var(--space-lg);
    border-top: 1px solid var(--color-border-secondary);
  }

  .tag-container {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-xs);
  }

  .autosave-indicator {
    display: flex;
    align-items: center;
    gap: var(--space-xs);
    font-size: var(--font-size-xs);
    color: var(--color-success);
  }

  /* ============================================
     EMPTY STATE
     ============================================ */

  .empty-state {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: var(--space-2xl);
    text-align: center;
  }

  .empty-icon {
    width: 64px;
    height: 64px;
    color: var(--color-text-muted);
    margin-bottom: var(--space-xl);
    opacity: 0.5;
  }

  .empty-state h3 {
    font-size: var(--font-size-xl);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-primary);
    margin-bottom: var(--space-sm);
  }

  .empty-state p {
    font-size: var(--font-size-sm);
    color: var(--color-text-secondary);
  }
</style>
