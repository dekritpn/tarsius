<script>
  import { onMount } from "svelte";
  import OutlineNode from "./OutlineNode.svelte";

  let projects = [];
  let selectedProject = null;
  let isCreating = false;
  let newProjectTitle = "";
  let newProjectTemplateId = "default";
  let newProjectOutputDir = "./output";

  let scratches = [];
  let scratchesSearch = "";
  let filteredScratches = [];

  let documentContent = "";

  onMount(async () => {
    console.log("ProjectsView mounted");
    await loadProjects();
    await loadScratches();
  });

  async function loadProjects() {
    try {
      if (window.__TAURI__) {
        const { invoke } = await import("@tauri-apps/api/tauri");
        projects = await invoke("list_projects");
      } else {
        projects = [
          {
            id: "1",
            title: "Research Paper",
            outline: {
              id: "root",
              title: "Document Root",
              content: null,
              children: [
                {
                  id: "intro",
                  title: "Introduction",
                  content: "Overview of the research topic",
                  children: [],
                  scratches: [],
                },
                {
                  id: "methods",
                  title: "Methodology",
                  content: null,
                  children: [],
                  scratches: [],
                },
              ],
              scratches: [],
            },
            settings: { template_id: "default", output_dir: "./output" },
            created_at: new Date().toISOString(),
            modified_at: new Date().toISOString(),
          },
        ];
      }
    } catch (e) {
      console.error("Error loading projects:", e);
      projects = [];
    }
  }

  async function loadScratches() {
    try {
      if (window.__TAURI__) {
        const { invoke } = await import("@tauri-apps/api/tauri");
        scratches = await invoke("list_scratches");
      } else {
        scratches = [
          {
            id: "1",
            title: "Introduction to LaTeX",
            content: "LaTeX content...",
            tags: ["latex"],
            created_at: new Date().toISOString(),
            modified_at: new Date().toISOString(),
          },
          {
            id: "2",
            title: "Research Notes",
            content: "Research content...",
            tags: ["research"],
            created_at: new Date().toISOString(),
            modified_at: new Date().toISOString(),
          },
        ];
      }
      filterScratches();
    } catch (e) {
      console.error("Error loading scratches:", e);
      scratches = [];
      filterScratches();
    }
  }

  function filterScratches() {
    if (!scratchesSearch) {
      filteredScratches = scratches;
    } else {
      const query = scratchesSearch.toLowerCase();
      filteredScratches = scratches.filter(
        (scratch) =>
          scratch.title.toLowerCase().includes(query) ||
          scratch.content.toLowerCase().includes(query) ||
          scratch.tags.some((tag) => tag.toLowerCase().includes(query)),
      );
    }
  }

  $: if (scratchesSearch !== undefined) filterScratches();

  async function createProject() {
    if (!newProjectTitle.trim()) return;
    try {
      if (window.__TAURI__) {
        const { invoke } = await import("@tauri-apps/api/tauri");
        const project = await invoke("create_project", {
          title: newProjectTitle,
          template_id: newProjectTemplateId,
          output_dir: newProjectOutputDir,
        });
        projects = [...projects, project];
      } else {
        const newProject = {
          id: Date.now().toString(),
          title: newProjectTitle,
          outline: {
            id: "root",
            title: "Document Root",
            content: null,
            children: [],
            scratches: [],
          },
          settings: {
            template_id: newProjectTemplateId,
            output_dir: newProjectOutputDir,
          },
          created_at: new Date().toISOString(),
          modified_at: new Date().toISOString(),
        };
        projects = [...projects, newProject];
      }
      newProjectTitle = "";
      isCreating = false;
    } catch (e) {
      console.error("Error creating project:", e);
    }
  }

  function selectProject(project) {
    selectedProject = project;
    updateDocumentContent();
  }

  function updateDocumentContent() {
    if (!selectedProject) return;
    function generateContent(node, level = 0) {
      let content = `${"#".repeat(level + 1)} ${node.title}\n\n`;
      if (node.content) {
        content += `${node.content}\n\n`;
      }
      for (let scratch of node.scratches) {
        if (scratch.mode === "Include") {
          const scratchData = scratches.find(
            (s) => s.id === scratch.scratch_id,
          );
          if (scratchData) {
            content += `${scratchData.content}\n\n`;
          }
        }
      }
      for (let child of node.children) {
        content += generateContent(child, level + 1);
      }
      return content;
    }
    documentContent = generateContent(selectedProject.outline);
  }

  async function saveProject() {
    try {
      if (window.__TAURI__) {
        const { invoke } = await import("@tauri-apps/api/tauri");
        await invoke("save_project", selectedProject);
      }
    } catch (e) {
      console.error("Error saving project:", e);
    }
  }

  function addChildToNode(event) {
    const nodeId = event.detail;
    function addChildRecursive(node) {
      if (node.id === nodeId) {
        const newChild = {
          id: Date.now().toString() + Math.random().toString(36).substr(2, 9),
          title: "New Section",
          content: null,
          children: [],
          scratches: [],
        };
        node.children = [...node.children, newChild];
        return true;
      }
      for (let child of node.children) {
        if (addChildRecursive(child)) return true;
      }
      return false;
    }
    addChildRecursive(selectedProject.outline);
    selectedProject = { ...selectedProject };
    updateDocumentContent();
    saveProject();
  }

  function handleDragStart(e, scratch) {
    e.dataTransfer.setData("application/json", JSON.stringify(scratch));
  }

  function handleDropScratch(event) {
    const nodeId = event.detail.nodeId;
    const scratch = JSON.parse(event.detail.scratchData);
    function addScratchRecursive(node) {
      if (node.id === nodeId) {
        const scratchLink = {
          scratch_id: scratch.id,
          mode: "Include",
          insertion: {
            body: true,
            footnote: false,
            reference: false,
            appendix: false,
          },
        };
        node.scratches = [...node.scratches, scratchLink];
        return true;
      }
      for (let child of node.children) {
        if (addScratchRecursive(child)) return true;
      }
      return false;
    }
    addScratchRecursive(selectedProject.outline);
    selectedProject = { ...selectedProject };
    updateDocumentContent();
    saveProject();
  }

  function handleNodeUpdated() {
    selectedProject = { ...selectedProject };
    updateDocumentContent();
    saveProject();
  }

  function handleDeleteNode(event) {
    const nodeId = event.detail;
    function removeNodeRecursive(node) {
      const index = node.children.findIndex((child) => child.id === nodeId);
      if (index !== -1) {
        node.children.splice(index, 1);
        return true;
      }
      for (let child of node.children) {
        if (removeNodeRecursive(child)) return true;
      }
      return false;
    }
    removeNodeRecursive(selectedProject.outline);
    selectedProject = { ...selectedProject };
    updateDocumentContent();
    saveProject();
  }
</script>

<div class="projects-view">
  <!-- Header -->
  <div class="view-header">
    <div class="header-content">
      <h1 class="view-title">Projects</h1>
      <p class="view-subtitle">
        {projects.length} project{projects.length !== 1 ? "s" : ""}
      </p>
    </div>
    <button class="btn btn-primary" on:click={() => (isCreating = !isCreating)}>
      <svg
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        style="width: 16px; height: 16px;"
      >
        <path d="M12 5v14m-7-7h14" />
      </svg>
      {isCreating ? "Cancel" : "New Project"}
    </button>
  </div>

  <!-- Creation Form -->
  {#if isCreating}
    <div class="create-form card animate-slide-up">
      <h3 class="form-title">Create New Project</h3>
      <input
        bind:value={newProjectTitle}
        placeholder="Project Title"
        class="input"
        style="margin-bottom: var(--space-md);"
      />
      <div
        style="display: flex; gap: var(--space-sm); justify-content: flex-end;"
      >
        <button class="btn btn-secondary" on:click={() => (isCreating = false)}
          >Cancel</button
        >
        <button class="btn btn-primary" on:click={createProject}>Create</button>
      </div>
    </div>
  {/if}

  <!-- Main Content -->
  <div class="main-content">
    <!-- Projects List -->
    <div class="projects-list">
      {#each projects as project}
        <div
          class="project-card card"
          class:selected={selectedProject && selectedProject.id === project.id}
          on:click={() => selectProject(project)}
        >
          <h3 class="project-title">{project.title}</h3>
          <p class="project-date">
            Modified {new Date(project.modified_at).toLocaleDateString()}
          </p>
        </div>
      {/each}
    </div>

    <!-- Project Detail -->
    {#if selectedProject}
      <div class="project-detail">
        <!-- Outline Panel -->
        <div class="outline-panel">
          <div class="panel-header">
            <h3 class="panel-title">Outline</h3>
          </div>
          <div class="outline-content">
            <OutlineNode
              node={selectedProject.outline}
              level={0}
              on:addChild={addChildToNode}
              on:dropScratch={handleDropScratch}
              on:nodeUpdated={handleNodeUpdated}
              on:deleteNode={handleDeleteNode}
            />
          </div>
        </div>

        <!-- Scratches Panel -->
        <div class="scratches-panel">
          <div class="panel-header">
            <h3 class="panel-title">Scratches</h3>
            <input
              bind:value={scratchesSearch}
              placeholder="Search..."
              class="input"
              style="margin-top: var(--space-sm);"
            />
          </div>
          <div class="scratches-list">
            {#each filteredScratches as scratch}
              <div
                class="scratch-item"
                draggable="true"
                on:dragstart={(e) => handleDragStart(e, scratch)}
              >
                <div class="scratch-item-title">{scratch.title}</div>
                <div class="scratch-item-preview">
                  {scratch.content.substring(0, 40)}...
                </div>
              </div>
            {/each}
          </div>
        </div>

        <!-- Document Preview -->
        <div class="document-panel">
          <div class="panel-header">
            <h3 class="panel-title">Document Preview</h3>
          </div>
          <div class="document-content">
            <pre class="document-text">{documentContent}</pre>
          </div>
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
            d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"
          />
        </svg>
        <h3>No Project Selected</h3>
        <p>Select a project to view and edit its outline</p>
      </div>
    {/if}
  </div>
</div>

<style>
  .projects-view {
    height: 100%;
    display: flex;
    flex-direction: column;
    background: var(--color-bg-primary);
  }

  .view-header {
    padding: var(--space-xl) var(--space-2xl);
    border-bottom: 1px solid var(--color-border-primary);
    background: var(--color-bg-secondary);
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .header-content h1 {
    font-size: var(--font-size-3xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text-primary);
    margin-bottom: var(--space-xs);
  }

  .view-subtitle {
    font-size: var(--font-size-sm);
    color: var(--color-text-secondary);
  }

  .create-form {
    margin: var(--space-xl) var(--space-2xl);
  }

  .form-title {
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-primary);
    margin-bottom: var(--space-lg);
  }

  .main-content {
    flex: 1;
    display: flex;
    overflow: hidden;
  }

  .projects-list {
    width: 280px;
    border-right: 1px solid var(--color-border-primary);
    overflow-y: auto;
    background: var(--color-bg-secondary);
    padding: var(--space-lg);
    display: flex;
    flex-direction: column;
    gap: var(--space-md);
  }

  .project-card {
    cursor: pointer;
    padding: var(--space-lg);
  }

  .project-card.selected {
    border-color: var(--color-accent-primary);
    background: var(--color-bg-tertiary);
    box-shadow: var(--shadow-glow);
  }

  .project-title {
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-primary);
    margin-bottom: var(--space-xs);
  }

  .project-date {
    font-size: var(--font-size-xs);
    color: var(--color-text-tertiary);
  }

  .project-detail {
    flex: 1;
    display: grid;
    grid-template-columns: 1fr 280px 1fr;
    gap: 1px;
    background: var(--color-border-primary);
  }

  .outline-panel,
  .scratches-panel,
  .document-panel {
    background: var(--color-bg-primary);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .panel-header {
    padding: var(--space-lg);
    border-bottom: 1px solid var(--color-border-primary);
    background: var(--color-bg-secondary);
  }

  .panel-title {
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-primary);
  }

  .outline-content {
    flex: 1;
    overflow-y: auto;
    padding: var(--space-lg);
  }

  .scratches-list {
    flex: 1;
    overflow-y: auto;
    padding: var(--space-lg);
    display: flex;
    flex-direction: column;
    gap: var(--space-sm);
  }

  .scratch-item {
    padding: var(--space-md);
    background: var(--color-bg-elevated);
    border: 1px solid var(--color-border-primary);
    border-radius: var(--radius-md);
    cursor: grab;
    transition: all var(--transition-fast);
  }

  .scratch-item:hover {
    border-color: var(--color-accent-primary);
    box-shadow: var(--shadow-md);
  }

  .scratch-item:active {
    cursor: grabbing;
  }

  .scratch-item-title {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    color: var(--color-text-primary);
    margin-bottom: var(--space-xs);
  }

  .scratch-item-preview {
    font-size: var(--font-size-xs);
    color: var(--color-text-secondary);
  }

  .document-content {
    flex: 1;
    overflow-y: auto;
    padding: var(--space-lg);
  }

  .document-text {
    font-family: var(--font-mono);
    font-size: var(--font-size-sm);
    color: var(--color-text-secondary);
    line-height: 1.8;
    white-space: pre-wrap;
    word-wrap: break-word;
  }

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
