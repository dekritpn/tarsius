<script>
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import OutlineNode from './OutlineNode.svelte';
  import MonacoEditor from 'svelte-monaco';

  let projects = [];
  let selectedProject = null;
  let isCreating = false;
  let newProjectTitle = '';
  let newProjectTemplateId = 'default'; // For now, assume a default template
  let newProjectOutputDir = './output';

  let scratches = [];
  let scratchesSearch = '';
  let filteredScratches = [];

  let documentContent = '';

  onMount(async () => {
    await loadProjects();
    await loadScratches();
  });

  async function loadProjects() {
    try {
      projects = await invoke('list_projects');
    } catch (e) {
      console.error('Error loading projects:', e);
    }
  }

  async function loadScratches() {
    try {
      scratches = await invoke('list_scratches');
      filterScratches();
    } catch (e) {
      console.error('Error loading scratches:', e);
    }
  }

  function filterScratches() {
    if (!scratchesSearch) {
      filteredScratches = scratches;
    } else {
      const query = scratchesSearch.toLowerCase();
      filteredScratches = scratches.filter(scratch =>
        scratch.title.toLowerCase().includes(query) ||
        scratch.content.toLowerCase().includes(query) ||
        scratch.tags.some(tag => tag.toLowerCase().includes(query))
      );
    }
  }

  $: if (scratchesSearch !== undefined) filterScratches();

  async function createProject() {
    if (!newProjectTitle.trim()) return;
    try {
      const project = await invoke('create_project', {
        title: newProjectTitle,
        template_id: newProjectTemplateId,
        output_dir: newProjectOutputDir
      });
      projects = [...projects, project];
      newProjectTitle = '';
      isCreating = false;
    } catch (e) {
      console.error('Error creating project:', e);
    }
  }

  function selectProject(project) {
    selectedProject = project;
    updateDocumentContent();
  }

  function updateDocumentContent() {
    if (!selectedProject) return;
    // Generate document content from outline
    function generateContent(node, level = 0) {
      let content = `${'#'.repeat(level + 1)} ${node.title}\n\n`;
      if (node.content) {
        content += `${node.content}\n\n`;
      }
      for (let scratch of node.scratches) {
        if (scratch.mode === 'Include') {
          // Load actual scratch content
          const scratchData = scratches.find(s => s.id === scratch.scratch_id);
          if (scratchData) {
            content += `${scratchData.content}\n\n`;
          } else {
            content += `**Scratch ${scratch.scratch_id} not found**\n\n`;
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
      await invoke('save_project', { projectDto: selectedProject });
    } catch (e) {
      console.error('Error saving project:', e);
    }
  }

  function addChildToNode(nodeId) {
    // Find the node and add a child
    function addChildRecursive(node) {
      if (node.id === nodeId) {
        const newChild = {
          id: crypto.randomUUID(),
          title: 'New Section',
          content: null,
          children: [],
          scratches: []
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
    selectedProject = { ...selectedProject }; // Trigger reactivity
    updateDocumentContent();
    saveProject();
  }

  function handleDragStart(e, scratch) {
    e.dataTransfer.setData('application/json', JSON.stringify(scratch));
  }

  function handleDropScratch(event) {
    const nodeId = event.detail.nodeId;
    const scratch = JSON.parse(event.detail.scratchData);
    // Add scratch to node
    function addScratchRecursive(node) {
      if (node.id === nodeId) {
        const scratchLink = {
          scratch_id: scratch.id,
          mode: 'Include',
          insertion: {
            body: true,
            footnote: false,
            reference: false,
            appendix: false
          }
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

  function handleNodeUpdated(event) {
    // Node was updated, just trigger reactivity and save
    selectedProject = { ...selectedProject };
    updateDocumentContent();
    saveProject();
  }

  function handleDeleteNode(event) {
    const nodeId = event.detail;
    // Remove node from outline
    function removeNodeRecursive(node) {
      const index = node.children.findIndex(child => child.id === nodeId);
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
  <div class="toolbar">
    <h2>Projects</h2>
    <button on:click={() => isCreating = !isCreating} class="create-btn">
      {isCreating ? 'Cancel' : 'New Project'}
    </button>
  </div>

  {#if isCreating}
    <div class="create-form">
      <input bind:value={newProjectTitle} placeholder="Project Title" required />
      <input bind:value={newProjectTemplateId} placeholder="Template ID" />
      <input bind:value={newProjectOutputDir} placeholder="Output Directory" />
      <button on:click={createProject}>Create</button>
    </div>
  {/if}

  <div class="projects-list">
    {#each projects as project}
      <div class="project-item" class:selected={selectedProject && selectedProject.id === project.id} on:click={() => selectProject(project)}>
        <h3>{project.title}</h3>
        <p>Created: {new Date(project.created_at).toLocaleDateString()}</p>
      </div>
    {/each}
  </div>

  {#if selectedProject}
    <div class="project-detail">
      <h3>{selectedProject.title}</h3>
      <div class="project-content">
        <div class="outline-panel">
          <h4>Outline</h4>
          <OutlineNode node={selectedProject.outline} level={0} on:addChild={addChildToNode} on:dropScratch={handleDropScratch} on:nodeUpdated={handleNodeUpdated} on:deleteNode={handleDeleteNode} />
        </div>
        <div class="scratches-panel">
          <h4>Scratches</h4>
          <input bind:value={scratchesSearch} placeholder="Search scratches..." class="search-input" />
          <div class="scratches-list">
            {#each filteredScratches as scratch}
              <div class="scratch-item" draggable="true" on:dragstart={(e) => handleDragStart(e, scratch)}>
                <div class="scratch-title">{scratch.title}</div>
                <div class="scratch-preview">{scratch.content.substring(0, 50)}...</div>
              </div>
            {/each}
          </div>
        </div>
        <div class="document-panel">
          <h4>Document</h4>
          <MonacoEditor
            bind:value={documentContent}
            language="markdown"
            theme="vs-dark"
            options={{
              readOnly: false,
              minimap: { enabled: false },
              wordWrap: 'on',
              automaticLayout: true
            }}
          />
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .projects-view {
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  .toolbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;
    border-bottom: 1px solid #ccc;
  }

  .create-btn {
    padding: 0.5rem 1rem;
  }

  .create-form {
    padding: 1rem;
    border-bottom: 1px solid #ccc;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .projects-list {
    flex: 1;
    padding: 1rem;
    overflow-y: auto;
  }

  .project-item {
    padding: 1rem;
    border: 1px solid #ddd;
    margin-bottom: 0.5rem;
    cursor: pointer;
  }

  .project-item:hover {
    background: #f5f5f5;
  }

  .project-item.selected {
    background: #e0e0e0;
  }

  .project-detail {
    flex: 1;
    padding: 1rem;
    border-top: 1px solid #ccc;
    display: flex;
    flex-direction: column;
  }

  .project-content {
    flex: 1;
    display: flex;
  }

  .outline-panel {
    flex: 1;
    padding: 1rem;
    border-right: 1px solid #ccc;
  }

  .scratches-panel {
    width: 300px;
    padding: 1rem;
    border-right: 1px solid #ccc;
  }

  .document-panel {
    flex: 1;
    padding: 1rem;
    display: flex;
    flex-direction: column;
  }

  .document-panel :global(.monaco-editor) {
    flex: 1;
  }

  .search-input {
    width: 100%;
    padding: 0.5rem;
    margin-bottom: 1rem;
  }

  .scratches-list {
    max-height: 400px;
    overflow-y: auto;
  }

  .scratch-item {
    padding: 0.5rem;
    border: 1px solid #ddd;
    margin-bottom: 0.5rem;
    cursor: grab;
  }

  .scratch-item:hover {
    background: #f5f5f5;
  }

  .scratch-title {
    font-weight: bold;
  }

  .scratch-preview {
    font-size: 0.9em;
    color: #666;
  }
</style>