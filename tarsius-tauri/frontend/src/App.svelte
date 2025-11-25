<script>
  // Temporarily remove all imports to test basic Svelte mounting
  // import ScratchesView from './ScratchesView.svelte';
  // import ProjectsView from './ProjectsView.svelte';

  let currentView = 'scratches';

  console.log('App.svelte script loaded - testing basic mounting without imports');

  function switchView(view) {
    console.log('Switching view to:', view);
    currentView = view;
  }

  // Debug: Show that component mounted
  console.log('App.svelte component created successfully');
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
        <div style="padding: 20px; background: #e8f5e8; border: 2px solid #4CAF50;">
          <h2>🗒️ Scratches View</h2>
          <p>Basic scratches content - no imports</p>
          <p>Current view: {currentView}</p>
        </div>
      {:else if currentView === 'projects'}
        <div style="padding: 20px; background: #e3f2fd; border: 2px solid #2196F3;">
          <h2>📁 Projects View</h2>
          <p>Basic projects content - no imports</p>
          <p>Current view: {currentView}</p>
        </div>
      {:else if currentView === 'latex'}
        <h1>LaTeX</h1>
        <p>LaTeX view - no imports</p>
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