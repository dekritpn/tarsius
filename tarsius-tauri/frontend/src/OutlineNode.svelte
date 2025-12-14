<script>
  import { createEventDispatcher } from "svelte";

  export let node;
  export let level = 0;

  const dispatch = createEventDispatcher();

  let isExpanded = true;
  let isEditing = false;
  let editTitle = node.title;
  let editContent = node.content || "";

  function toggleExpanded() {
    isExpanded = !isExpanded;
  }

  function addChild() {
    dispatch("addChild", node.id);
  }

  function startEdit() {
    isEditing = true;
    editTitle = node.title;
    editContent = node.content || "";
  }

  function saveEdit() {
    node.title = editTitle;
    node.content = editContent || null;
    isEditing = false;
    dispatch("nodeUpdated", node);
  }

  function cancelEdit() {
    isEditing = false;
  }

  function deleteNode() {
    dispatch("deleteNode", node.id);
  }

  function removeScratch(scratchId) {
    node.scratches = node.scratches.filter((s) => s.scratch_id !== scratchId);
    dispatch("nodeUpdated", node);
  }

  function handleDragOver(e) {
    e.preventDefault();
    e.currentTarget.classList.add("drag-over");
  }

  function handleDragLeave(e) {
    e.currentTarget.classList.remove("drag-over");
  }

  function handleDrop(e) {
    e.preventDefault();
    e.currentTarget.classList.remove("drag-over");
    const scratchData = e.dataTransfer.getData("application/json");
    dispatch("dropScratch", { nodeId: node.id, scratchData });
  }
</script>

<div
  class="outline-node"
  style="margin-left: {level * 24}px"
  on:dragover={handleDragOver}
  on:dragleave={handleDragLeave}
  on:drop={handleDrop}
>
  <div class="node-header">
    {#if node.children.length > 0}
      <button on:click={toggleExpanded} class="expand-btn">
        <svg
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          class="expand-icon"
          class:expanded={isExpanded}
        >
          <path d="M9 5l7 7-7 7" />
        </svg>
      </button>
    {:else}
      <div class="expand-spacer"></div>
    {/if}

    {#if isEditing}
      <input
        bind:value={editTitle}
        class="edit-title input"
        placeholder="Section title"
      />
      <button on:click={saveEdit} class="action-btn save-btn" title="Save">
        <svg
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          <path d="M5 13l4 4L19 7" />
        </svg>
      </button>
      <button
        on:click={cancelEdit}
        class="action-btn cancel-btn"
        title="Cancel"
      >
        <svg
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          <path d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    {:else}
      <span class="node-title">{node.title}</span>
      <div class="node-actions">
        <button on:click={addChild} class="action-btn" title="Add child">
          <svg
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          >
            <path d="M12 5v14m-7-7h14" />
          </svg>
        </button>
        <button on:click={startEdit} class="action-btn" title="Edit">
          <svg
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          >
            <path
              d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"
            />
          </svg>
        </button>
        <button
          on:click={deleteNode}
          class="action-btn delete-btn"
          title="Delete"
        >
          <svg
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          >
            <path
              d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
            />
          </svg>
        </button>
      </div>
    {/if}
  </div>

  {#if isEditing}
    <textarea
      bind:value={editContent}
      class="edit-content input"
      placeholder="Section content..."
    ></textarea>
  {:else if node.content}
    <div class="node-content">{node.content}</div>
  {/if}

  {#if node.scratches.length > 0}
    <div class="node-scratches">
      {#each node.scratches as scratch}
        <div class="scratch-link">
          <svg
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            class="link-icon"
          >
            <path
              d="M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1"
            />
          </svg>
          <span class="link-mode">{scratch.mode}</span>
          <span class="link-id">{scratch.scratch_id}</span>
          <button
            on:click={() => removeScratch(scratch.scratch_id)}
            class="remove-scratch-btn"
            title="Remove"
          >
            <svg
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
            >
              <path d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
      {/each}
    </div>
  {/if}

  {#if isExpanded}
    {#each node.children as child}
      <svelte:self
        node={child}
        level={level + 1}
        on:addChild
        on:dropScratch
        on:nodeUpdated
        on:deleteNode
      />
    {/each}
  {/if}
</div>

<style>
  .outline-node {
    margin-bottom: var(--space-sm);
    transition: all var(--transition-fast);
  }

  .outline-node.drag-over {
    background: rgba(99, 102, 241, 0.1);
    border-left: 2px solid var(--color-accent-primary);
    padding-left: var(--space-sm);
  }

  .node-header {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-sm) var(--space-md);
    background: var(--color-bg-elevated);
    border: 1px solid var(--color-border-primary);
    border-radius: var(--radius-md);
    transition: all var(--transition-fast);
  }

  .node-header:hover {
    border-color: var(--color-border-accent);
    background: var(--color-bg-tertiary);
  }

  .expand-btn {
    background: none;
    border: none;
    cursor: pointer;
    padding: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--color-text-secondary);
    transition: all var(--transition-fast);
  }

  .expand-btn:hover {
    color: var(--color-accent-primary);
  }

  .expand-icon {
    width: 16px;
    height: 16px;
    transition: transform var(--transition-fast);
  }

  .expand-icon.expanded {
    transform: rotate(90deg);
  }

  .expand-spacer {
    width: 16px;
    height: 16px;
  }

  .node-title {
    flex: 1;
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-primary);
    font-size: var(--font-size-sm);
  }

  .node-actions {
    display: flex;
    gap: var(--space-xs);
    opacity: 0;
    transition: opacity var(--transition-fast);
  }

  .node-header:hover .node-actions {
    opacity: 1;
  }

  .action-btn {
    background: transparent;
    border: 1px solid var(--color-border-primary);
    border-radius: var(--radius-sm);
    cursor: pointer;
    padding: var(--space-xs);
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--color-text-secondary);
    transition: all var(--transition-fast);
  }

  .action-btn:hover {
    background: var(--color-bg-secondary);
    border-color: var(--color-accent-primary);
    color: var(--color-accent-primary);
  }

  .action-btn svg {
    width: 14px;
    height: 14px;
  }

  .save-btn:hover {
    color: var(--color-success);
    border-color: var(--color-success);
  }

  .cancel-btn:hover {
    color: var(--color-text-tertiary);
  }

  .delete-btn:hover {
    color: var(--color-error);
    border-color: var(--color-error);
  }

  .edit-title {
    flex: 1;
    font-weight: var(--font-weight-semibold);
    font-size: var(--font-size-sm);
  }

  .edit-content {
    margin-top: var(--space-sm);
    margin-left: calc(16px + var(--space-sm));
    width: calc(100% - 16px - var(--space-sm));
    min-height: 60px;
    resize: vertical;
    font-family: var(--font-mono);
    font-size: var(--font-size-sm);
  }

  .node-content {
    margin-top: var(--space-sm);
    margin-left: calc(16px + var(--space-sm));
    padding: var(--space-md);
    font-size: var(--font-size-sm);
    color: var(--color-text-secondary);
    background: var(--color-bg-tertiary);
    border-radius: var(--radius-md);
    border: 1px solid var(--color-border-secondary);
  }

  .node-scratches {
    margin-top: var(--space-sm);
    margin-left: calc(16px + var(--space-sm));
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
  }

  .scratch-link {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-xs) var(--space-sm);
    background: var(--color-bg-tertiary);
    border: 1px solid var(--color-border-secondary);
    border-radius: var(--radius-sm);
    font-size: var(--font-size-xs);
    color: var(--color-text-tertiary);
    transition: all var(--transition-fast);
  }

  .scratch-link:hover {
    border-color: var(--color-accent-primary);
    background: var(--color-bg-elevated);
  }

  .link-icon {
    width: 12px;
    height: 12px;
    color: var(--color-accent-primary);
  }

  .link-mode {
    font-weight: var(--font-weight-medium);
    color: var(--color-accent-primary);
  }

  .link-id {
    flex: 1;
    font-family: var(--font-mono);
  }

  .remove-scratch-btn {
    background: none;
    border: none;
    color: var(--color-text-muted);
    cursor: pointer;
    padding: 0;
    display: flex;
    align-items: center;
    transition: color var(--transition-fast);
  }

  .remove-scratch-btn:hover {
    color: var(--color-error);
  }

  .remove-scratch-btn svg {
    width: 12px;
    height: 12px;
  }
</style>
