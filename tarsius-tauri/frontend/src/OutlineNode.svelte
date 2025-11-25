<script>
  import { createEventDispatcher } from 'svelte';

  export let node;
  export let level = 0;

  const dispatch = createEventDispatcher();

  let isExpanded = true;
  let isEditing = false;
  let editTitle = node.title;
  let editContent = node.content || '';

  function toggleExpanded() {
    isExpanded = !isExpanded;
  }

  function addChild() {
    dispatch('addChild', node.id);
  }

  function startEdit() {
    isEditing = true;
    editTitle = node.title;
    editContent = node.content || '';
  }

  function saveEdit() {
    node.title = editTitle;
    node.content = editContent || null;
    isEditing = false;
    dispatch('nodeUpdated', node);
  }

  function cancelEdit() {
    isEditing = false;
  }

  function deleteNode() {
    dispatch('deleteNode', node.id);
  }

  function removeScratch(scratchId) {
    node.scratches = node.scratches.filter(s => s.scratch_id !== scratchId);
    dispatch('nodeUpdated', node);
  }

  function handleDragOver(e) {
    e.preventDefault();
  }

  function handleDrop(e) {
    e.preventDefault();
    const scratchData = e.dataTransfer.getData('application/json');
    dispatch('dropScratch', { nodeId: node.id, scratchData });
  }
</script>

<div class="outline-node" style="margin-left: {level * 20}px" on:dragover={handleDragOver} on:drop={handleDrop}>
  <div class="node-header">
    {#if node.children.length > 0}
      <button on:click={toggleExpanded} class="expand-btn">
        {isExpanded ? '▼' : '▶'}
      </button>
    {/if}
    {#if isEditing}
      <input bind:value={editTitle} class="edit-title" />
      <button on:click={saveEdit} class="action-btn">✓</button>
      <button on:click={cancelEdit} class="action-btn">✗</button>
    {:else}
      <span class="node-title">{node.title}</span>
      <button on:click={addChild} class="action-btn">+</button>
      <button on:click={startEdit} class="action-btn">✎</button>
      <button on:click={deleteNode} class="action-btn delete-btn">🗑</button>
    {/if}
  </div>

  {#if isEditing}
    <textarea bind:value={editContent} class="edit-content" placeholder="Node content..."></textarea>
  {:else if node.content}
    <div class="node-content">{node.content}</div>
  {/if}

  {#if node.scratches.length > 0}
    <div class="node-scratches">
      {#each node.scratches as scratch}
        <div class="scratch-link">
          <span>{scratch.mode}: {scratch.scratch_id}</span>
          <button on:click={() => removeScratch(scratch.scratch_id)} class="remove-scratch-btn">×</button>
        </div>
      {/each}
    </div>
  {/if}

  {#if isExpanded}
    {#each node.children as child}
      <svelte:self node={child} level={level + 1} />
    {/each}
  {/if}
</div>

<style>
  .outline-node {
    margin-bottom: 0.5rem;
  }

  .node-header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .expand-btn {
    background: none;
    border: none;
    cursor: pointer;
    font-size: 0.8em;
  }

  .node-title {
    flex: 1;
    font-weight: bold;
  }

  .action-btn {
    background: none;
    border: 1px solid #ccc;
    cursor: pointer;
    padding: 0.2rem 0.4rem;
    font-size: 0.8em;
  }

  .delete-btn {
    color: #d00;
  }

  .edit-title {
    flex: 1;
    padding: 0.2rem;
    border: 1px solid #ccc;
    font-weight: bold;
  }

  .edit-content {
    margin-left: 1rem;
    width: calc(100% - 1rem);
    padding: 0.5rem;
    border: 1px solid #ccc;
    min-height: 60px;
    resize: vertical;
  }

  .node-content {
    margin-left: 1rem;
    font-size: 0.9em;
    color: #666;
  }

  .node-scratches {
    margin-left: 1rem;
  }

  .scratch-link {
    font-size: 0.8em;
    color: #888;
    margin-bottom: 0.2rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .remove-scratch-btn {
    background: none;
    border: none;
    color: #d00;
    cursor: pointer;
    font-size: 1.2em;
    padding: 0 0.2rem;
  }
</style>