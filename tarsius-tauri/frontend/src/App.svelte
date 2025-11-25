<script>
  import ScratchesView from './ScratchesView.svelte';
  import ProjectsView from './ProjectsView.svelte';

  let currentView = 'scratches';

  function switchView(view) {
    currentView = view;
  }
</script>

<main>
  <aside>
    <nav>
      <button on:click={() => switchView('scratches')}>Scratches</button>
      <button on:click={() => switchView('projects')}>Projects</button>
      <button on:click={() => switchView('latex')}>LaTeX</button>
    </nav>
  </aside>
   <section>
      {#if currentView === 'scratches'}
        <ScratchesView />
      {:else if currentView === 'projects'}
        <ProjectsView />
      {:else if currentView === 'latex'}
        <h1>LaTeX</h1>
        <p>LaTeX view</p>
      {/if}
   </section>
</main>

<style>
  main {
    display: flex;
    height: 100vh;
  }
  aside {
    width: 200px;
    background: #f0f0f0;
    padding: 1rem;
  }
  section {
    flex: 1;
    padding: 1rem;
  }
  nav {
    display: flex;
    flex-direction: column;
  }
  button {
    margin-bottom: 0.5rem;
  }
</style>