<script>
  import { onMount } from 'svelte';

  let scratches = [];
  let selectedScratch = null;
  let searchQuery = '';
  let filteredScratches = [];
  let editorContent = '';
  let isCreating = false;
  let newScratchTitle = '';
  let newScratchContent = '';
  let autosaveTimeout;

  // Check if we're in Tauri environment
  let isTauri = typeof window !== 'undefined' && window.__TAURI__;
  console.log('ScratchesView: Tauri environment detected:', isTauri);

  onMount(async () => {
    console.log('ScratchesView mounted');
    await loadScratches();
  });

  async function loadScratches() {
    try {
      if (isTauri) {
        const { invoke } = await import('@tauri-apps/api/tauri');
        console.log('Loading scratches from Tauri...');
        scratches = await invoke('list_scratches');
        console.log('Loaded scratches from Tauri:', scratches);
      } else {
        console.log('Using mock scratches data (not in Tauri environment)');
        // Mock data for development
        scratches = [
          { id: '1', title: 'Test Scratch 1', content: 'This is test content 1', tags: ['test'], created_at: new Date().toISOString(), modified_at: new Date().toISOString() },
          { id: '2', title: 'Test Scratch 2', content: 'This is test content 2', tags: ['demo'], created_at: new Date().toISOString(), modified_at: new Date().toISOString() }
        ];
      }
      filterScratches();
    } catch (e) {
      console.error('Error loading scratches:', e);
      // Fallback to mock data
      scratches = [
        { id: '1', title: 'Test Scratch 1', content: 'This is test content 1', tags: ['test'], created_at: new Date().toISOString(), modified_at: new Date().toISOString() },
        { id: '2', title: 'Test Scratch 2', content: 'This is test content 2', tags: ['demo'], created_at: new Date().toISOString(), modified_at: new Date().toISOString() }
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
      if (isTauri) {
        const { invoke } = await import('@tauri-apps/api/tauri');
        await invoke('create_scratch', {
          title: newScratchTitle,
          content: newScratchContent,
          tags: [],
          source: null
        });
        await loadScratches();
      } else {
        // Mock create for development
        const newScratch = {
          id: Date.now().toString(),
          title: newScratchTitle,
          content: newScratchContent,
          tags: [],
          created_at: new Date().toISOString(),
          modified_at: new Date().toISOString()
        };
        scratches = [...scratches, newScratch];
        filterScratches();
      }
    } catch (e) {
      console.error('Error creating scratch:', e);
    }

    newScratchTitle = '';
    newScratchContent = '';
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
        const { invoke } = await import('@tauri-apps/api/tauri');
        await invoke('update_scratch', {
          id: selectedScratch.id,
          title: selectedScratch.title,
          content: editorContent,
          tags: selectedScratch.tags,
          source: selectedScratch.source
        });
      } else {
        // Mock update for development
        scratches = scratches.map(s =>
          s.id === selectedScratch.id
            ? { ...s, content: editorContent, modified_at: new Date().toISOString() }
            : s
        );
      }
    } catch (e) {
      console.error('Error updating scratch:', e);
    }
  }

</script>

<div style="padding: 20px; background: #f0f8ff; border: 2px solid #4CAF50;">
  <h3>🗒️ Scratches View - Basic Test</h3>
  <p>Component loaded successfully!</p>
  <p>Scratches count: {filteredScratches.length}</p>

  {#each filteredScratches as scratch}
    <div style="margin: 10px 0; padding: 10px; background: white; border: 1px solid #ddd;">
      <strong>{scratch.title}</strong>
      <p>{scratch.content}</p>
      <small>Tags: {scratch.tags.join(', ')}</small>
    </div>
  {/each}
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