<script>
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';

  let name = 'world';
  let scratches = [];
  let currentView = 'scratches';
  let newTitle = '';
  let newContent = '';

  onMount(async () => {
    // Test roundtrip
    try {
      scratches = await invoke('list_scratches');
      console.log('Scratches:', scratches);
    } catch (e) {
      console.error('Error:', e);
    }
  });

  async function createScratch() {
    try {
      const scratch = await invoke('create_scratch', { title: newTitle, content: newContent, tags: [], source: null });
      scratches = [...scratches, scratch];
      newTitle = '';
      newContent = '';
    } catch (e) {
      console.error('Error creating scratch:', e);
    }
  }

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
      <h1>Scratches</h1>
      <form on:submit|preventDefault={createScratch}>
        <input bind:value={newTitle} placeholder="Title" required />
        <textarea bind:value={newContent} placeholder="Content"></textarea>
        <button type="submit">Create Scratch</button>
      </form>
      <ul>
        {#each scratches as scratch}
          <li>{scratch.title}: {scratch.content}</li>
        {/each}
      </ul>
    {:else if currentView === 'projects'}
      <h1>Projects</h1>
      <p>Projects view</p>
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