<script>
  // Import both views with conditional loading
  import ScratchesView from './ScratchesView.svelte';
  import ProjectsView from './ProjectsView.svelte';

  let currentView = 'scratches';

  console.log('App.svelte script loaded - testing ScratchesView import');

  function switchView(view) {
    console.log('Switching view to:', view);
    currentView = view;
  }

  // Debug: Show that component mounted
  console.log('App.svelte component created');
</script>

<main>
  <!-- Debug: Always visible header -->
  <div style="background: #007acc; color: white; padding: 10px; font-weight: bold;">
    🐢 Tarsius App Loaded Successfully! Current view: {currentView}
  </div>

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
   :global(body) {
     margin: 0;
     padding: 0;
     font-family: Arial, sans-serif;
     background: #fff;
   }

   main {
     display: flex;
     height: 100vh;
     background: #f8f9fa;
   }

   aside {
     width: 200px;
     background: #f0f0f0;
     padding: 1rem;
     border-right: 1px solid #ddd;
   }

   section {
     flex: 1;
     padding: 1rem;
     background: white;
   }

   nav {
     display: flex;
     flex-direction: column;
   }

   button {
     margin-bottom: 0.5rem;
     padding: 0.5rem 1rem;
     border: 1px solid #ccc;
     background: white;
     cursor: pointer;
   }

   button:hover {
     background: #e9ecef;
   }

   h1 {
     color: #333;
     margin-top: 0;
   }
</style>