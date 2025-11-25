<script>
  import { onMount } from 'svelte';

  // Components will be loaded dynamically
  let ScratchesView = null;
  let ProjectsView = null;
  let componentsLoaded = false;

  let currentView = 'scratches';

  console.log('App.svelte script loaded');

  onMount(async () => {
    console.log('App.svelte onMount - loading components...');

    try {
      const [scratchesModule, projectsModule] = await Promise.allSettled([
        import('./ScratchesView.svelte'),
        import('./ProjectsView.svelte')
      ]);

      if (scratchesModule.status === 'fulfilled') {
        ScratchesView = scratchesModule.value.default;
        console.log('ScratchesView loaded successfully');
      } else {
        console.error('Failed to load ScratchesView:', scratchesModule.reason.message);
      }

      if (projectsModule.status === 'fulfilled') {
        ProjectsView = projectsModule.value.default;
        console.log('ProjectsView loaded successfully');
      } else {
        console.error('Failed to load ProjectsView:', projectsModule.reason.message);
      }

      componentsLoaded = true;
      console.log('Component loading complete');
    } catch (e) {
      console.error('Error loading components:', e);
    }
  });

  function switchView(view) {
    console.log('Switching view to:', view);
    currentView = view;
  }

  // Debug: Show that component mounted
  console.log('App.svelte component created');
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
        {#if componentsLoaded && ScratchesView}
          <ScratchesView />
        {:else}
          <div style="padding: 20px; background: #e8f5e8; border: 2px solid #4CAF50;">
            <h2>🗒️ Scratches View</h2>
            <p>Loading ScratchesView component...</p>
            <p>Components loaded: {componentsLoaded ? 'Yes' : 'No'}</p>
            <p>ScratchesView available: {ScratchesView ? 'Yes' : 'No'}</p>
          </div>
        {/if}
      {:else if currentView === 'projects'}
        {#if componentsLoaded && ProjectsView}
          <ProjectsView />
        {:else}
          <div style="padding: 20px; background: #e3f2fd; border: 2px solid #2196F3;">
            <h2>📁 Projects View</h2>
            <p>Loading ProjectsView component...</p>
            <p>Components loaded: {componentsLoaded ? 'Yes' : 'No'}</p>
            <p>ProjectsView available: {ProjectsView ? 'Yes' : 'No'}</p>
          </div>
        {/if}
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