<script>
  import ScratchesView from "./ScratchesView.svelte";
  import ProjectsView from "./ProjectsView.svelte";

  let currentView = "scratches";

  console.log("Tarsius App loaded");

  function switchView(view) {
    console.log("Switching to view:", view);
    currentView = view;
  }
</script>

<main class="app-container">
  <!-- Modern Sidebar -->
  <aside class="sidebar">
    <div class="sidebar-header">
      <div class="logo">
        <div class="logo-icon">T</div>
        <span class="logo-text">Tarsius</span>
      </div>
    </div>

    <nav class="nav">
      <button
        class="nav-item"
        class:active={currentView === "scratches"}
        on:click={() => switchView("scratches")}
      >
        <svg
          class="nav-icon"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          <path
            d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"
          />
        </svg>
        <span class="nav-label">Scratches</span>
      </button>

      <button
        class="nav-item"
        class:active={currentView === "projects"}
        on:click={() => switchView("projects")}
      >
        <svg
          class="nav-icon"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          <path
            d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"
          />
        </svg>
        <span class="nav-label">Projects</span>
      </button>

      <button
        class="nav-item"
        class:active={currentView === "latex"}
        on:click={() => switchView("latex")}
      >
        <svg
          class="nav-icon"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          <path
            d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
          />
        </svg>
        <span class="nav-label">LaTeX</span>
      </button>
    </nav>

    <div class="sidebar-footer">
      <div class="status-indicator">
        <div class="status-dot"></div>
        <span class="status-text">Ready</span>
      </div>
    </div>
  </aside>

  <!-- Main Content Area -->
  <section class="content-area">
    {#if currentView === "scratches"}
      <ScratchesView />
    {:else if currentView === "projects"}
      <ProjectsView />
    {:else if currentView === "latex"}
      <div class="latex-placeholder">
        <div class="placeholder-content">
          <svg
            class="placeholder-icon"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          >
            <path
              d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
            />
          </svg>
          <h2>LaTeX Compilation</h2>
          <p>Live PDF preview and compilation coming soon</p>
          <span class="badge">Phase 7</span>
        </div>
      </div>
    {/if}
  </section>
</main>

<style>
  .app-container {
    display: flex;
    height: 100vh;
    width: 100vw;
    overflow: hidden;
    background: var(--gradient-dark);
  }

  /* ============================================
     SIDEBAR STYLES
     ============================================ */

  .sidebar {
    width: 240px;
    background: var(--color-bg-secondary);
    border-right: 1px solid var(--color-border-primary);
    display: flex;
    flex-direction: column;
    position: relative;
    z-index: var(--z-sticky);
  }

  .sidebar::before {
    content: "";
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 200px;
    background: var(--gradient-primary);
    opacity: 0.05;
    pointer-events: none;
  }

  .sidebar-header {
    padding: var(--space-xl) var(--space-lg);
    border-bottom: 1px solid var(--color-border-secondary);
  }

  .logo {
    display: flex;
    align-items: center;
    gap: var(--space-md);
  }

  .logo-icon {
    width: 40px;
    height: 40px;
    background: var(--gradient-primary);
    border-radius: var(--radius-lg);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: var(--font-size-xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text-primary);
    box-shadow: var(--shadow-glow);
  }

  .logo-text {
    font-size: var(--font-size-xl);
    font-weight: var(--font-weight-bold);
    background: var(--gradient-primary);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  /* Navigation */
  .nav {
    flex: 1;
    padding: var(--space-lg);
    display: flex;
    flex-direction: column;
    gap: var(--space-sm);
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: var(--space-md);
    padding: var(--space-md) var(--space-lg);
    background: transparent;
    border: none;
    border-radius: var(--radius-md);
    color: var(--color-text-secondary);
    font-family: var(--font-family);
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    cursor: pointer;
    transition: all var(--transition-fast);
    position: relative;
    overflow: hidden;
  }

  .nav-item::before {
    content: "";
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;
    width: 3px;
    background: var(--gradient-primary);
    transform: scaleY(0);
    transition: transform var(--transition-fast);
  }

  .nav-item:hover {
    background: var(--color-bg-tertiary);
    color: var(--color-text-primary);
  }

  .nav-item.active {
    background: var(--color-bg-elevated);
    color: var(--color-text-primary);
    box-shadow: var(--shadow-md);
  }

  .nav-item.active::before {
    transform: scaleY(1);
  }

  .nav-icon {
    width: 20px;
    height: 20px;
    flex-shrink: 0;
  }

  .nav-label {
    flex: 1;
    text-align: left;
  }

  /* Sidebar Footer */
  .sidebar-footer {
    padding: var(--space-lg);
    border-top: 1px solid var(--color-border-secondary);
  }

  .status-indicator {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-sm) var(--space-md);
    background: var(--color-bg-tertiary);
    border-radius: var(--radius-md);
  }

  .status-dot {
    width: 8px;
    height: 8px;
    background: var(--color-success);
    border-radius: 50%;
    animation: pulse 2s infinite;
  }

  .status-text {
    font-size: var(--font-size-xs);
    color: var(--color-text-secondary);
    font-weight: var(--font-weight-medium);
  }

  /* ============================================
     CONTENT AREA
     ============================================ */

  .content-area {
    flex: 1;
    overflow: hidden;
    position: relative;
  }

  /* LaTeX Placeholder */
  .latex-placeholder {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    padding: var(--space-2xl);
  }

  .placeholder-content {
    text-align: center;
    max-width: 400px;
  }

  .placeholder-icon {
    width: 80px;
    height: 80px;
    margin: 0 auto var(--space-xl);
    color: var(--color-accent-primary);
    opacity: 0.5;
  }

  .placeholder-content h2 {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text-primary);
    margin-bottom: var(--space-md);
  }

  .placeholder-content p {
    font-size: var(--font-size-base);
    color: var(--color-text-secondary);
    margin-bottom: var(--space-lg);
  }

  .badge {
    display: inline-block;
    padding: var(--space-xs) var(--space-md);
    background: var(--gradient-primary);
    color: var(--color-text-primary);
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-semibold);
    border-radius: var(--radius-full);
  }
</style>
