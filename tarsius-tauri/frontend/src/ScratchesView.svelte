<script>
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import VirtualList from 'svelte-virtual-list';
  import MonacoEditor from 'svelte-monaco';

  let scratches = [];
  let selectedScratch = null;
  let searchQuery = '';
  let filteredScratches = [];
  let editorContent = '';
  let isCreating = false;
  let newScratchTitle = '';
  let newScratchContent = '';
  let autosaveTimeout;

  onMount(async () => {
    await loadScratches();
  });

  async function loadScratches() {
    try {
      scratches = await invoke('list_scratches');
      filteredScratches = scratches;
    } catch (e) {
      console.error('Error loading scratches:', e);
    }
  }

  function filterScratches() {
    if (!searchQuery) {
      filteredScratches = scratches;
    } else {
      const query = searchQuery.toLowerCase();
      filteredScratches = scratches.filter(scratch =>
        scratch.title.toLowerCase().includes(query) ||
        scratch.content.toLowerCase().includes(query) ||
        scratch.tags.some(tag => tag.toLowerCase().includes(query))
      );
    }
  }

  $: if (searchQuery !== undefined) filterScratches();

  async function createScratch() {
    if (!newScratchTitle.trim()) return;
    try {
      const scratch = await invoke('create_scratch', {
        title: newScratchTitle,
        content: newScratchContent,
        tags: [],
        source: null
      });
      scratches = [...scratches, scratch];
      newScratchTitle = '';
      newScratchContent = '';
      isCreating = false;
    } catch (e) {
      console.error('Error creating scratch:', e);
    }
  }

  function selectScratch(scratch) {
    selectedScratch = scratch;
    editorContent = scratch.content;
  }

  async function updateScratch() {
    if (!selectedScratch) return;
    try {
      const updated = await invoke('update_scratch', {
        id: selectedScratch.id,
        content: editorContent
      });
      scratches = scratches.map(s => s.id === updated.id ? updated : s);
      selectedScratch = updated;
    } catch (e) {
      console.error('Error updating scratch:', e);
    }
  }

  function handleEditorChange(event) {
    editorContent = event.detail.value;
    clearTimeout(autosaveTimeout);
    autosaveTimeout = setTimeout(updateScratch, 1000); // Autosave after 1 second
  }

  async function deleteScratch(scratch) {
    if (!confirm(`Delete "${scratch.title}"?`)) return;
    try {
      await invoke('delete_scratch', { id: scratch.id });
      scratches = scratches.filter(s => s.id !== scratch.id);
      if (selectedScratch && selectedScratch.id === scratch.id) {
        selectedScratch = null;
        editorContent = '';
      }
    } catch (e) {
      console.error('Error deleting scratch:', e);
    }
  }
</script>

<div class="scratches-view">
  <div class="toolbar">
    <input bind:value={searchQuery} placeholder="Search scratches..." class="search-input" />
    <button on:click={() => isCreating = !isCreating} class="create-btn">
      {isCreating ? 'Cancel' : 'New Scratch'}
    </button>
  </div>

  {#if isCreating}
    <div class="create-form">
      <input bind:value={newScratchTitle} placeholder="Title" required />
      <textarea bind:value={newScratchContent} placeholder="Content" rows="5"></textarea>
      <button on:click={createScratch}>Create</button>
    </div>
  {/if}

  <div class="main-content">
    <div class="list-panel">
      <h3>Scratches ({filteredScratches.length})</h3>
      <VirtualList items={filteredScratches} let:item height="400px">
        <div class="scratch-item" class:selected={selectedScratch && selectedScratch.id === item.id} on:click={() => selectScratch(item)}>
          <div class="scratch-title">{item.title}</div>
          <div class="scratch-preview">{item.content.substring(0, 100)}...</div>
          <div class="scratch-meta">
            <span>{new Date(item.created_at).toLocaleDateString()}</span>
            {#if item.tags.length > 0}
              <span>{item.tags.join(', ')}</span>
            {/if}
          </div>
          <button on:click|stopPropagation={() => deleteScratch(item)} class="delete-btn">×</button>
        </div>
      </VirtualList>
    </div>

    {#if selectedScratch}
      <div class="editor-panel">
        <div class="editor-header">
          <h3>{selectedScratch.title}</h3>
        </div>
        <MonacoEditor
          bind:value={editorContent}
          on:change={handleEditorChange}
          language="markdown"
          theme="vs-dark"
          options={{
            minimap: { enabled: false },
            wordWrap: 'on',
            automaticLayout: true
          }}
        />
        <div class="metadata-panel">
          <div><strong>Created:</strong> {new Date(selectedScratch.created_at).toLocaleString()}</div>
          <div><strong>Modified:</strong> {new Date(selectedScratch.modified_at).toLocaleString()}</div>
          <div><strong>Tags:</strong> {selectedScratch.tags.join(', ') || 'None'}</div>
          <div><strong>Source:</strong> {selectedScratch.source || 'None'}</div>
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  .scratches-view {
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  .toolbar {
    display: flex;
    gap: 1rem;
    padding: 1rem;
    border-bottom: 1px solid #ccc;
  }

  .search-input {
    flex: 1;
    padding: 0.5rem;
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

  .create-form textarea {
    resize: vertical;
  }

  .main-content {
    flex: 1;
    display: flex;
  }

  .list-panel {
    width: 300px;
    border-right: 1px solid #ccc;
    padding: 1rem;
    overflow: hidden;
  }

  .scratch-item {
    padding: 1rem;
    border: 1px solid #ddd;
    margin-bottom: 0.5rem;
    cursor: pointer;
    position: relative;
  }

  .scratch-item:hover {
    background: #f5f5f5;
  }

  .scratch-item.selected {
    background: #e0e0e0;
  }

  .scratch-title {
    font-weight: bold;
  }

  .scratch-preview {
    font-size: 0.9em;
    color: #666;
    margin: 0.5rem 0;
  }

  .scratch-meta {
    font-size: 0.8em;
    color: #888;
    display: flex;
    justify-content: space-between;
  }

  .delete-btn {
    position: absolute;
    top: 0.5rem;
    right: 0.5rem;
    background: #f00;
    color: white;
    border: none;
    border-radius: 50%;
    width: 20px;
    height: 20px;
    cursor: pointer;
  }

  .editor-panel {
    flex: 1;
    display: flex;
    flex-direction: column;
    padding: 1rem;
  }

  .editor-header {
    margin-bottom: 1rem;
  }

  .metadata-panel {
    margin-top: 1rem;
    padding: 1rem;
    background: #f9f9f9;
    border-top: 1px solid #ccc;
    font-size: 0.9em;
  }

  .metadata-panel div {
    margin-bottom: 0.5rem;
  }
</style>