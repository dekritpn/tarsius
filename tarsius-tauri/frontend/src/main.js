import App from './App.svelte';

console.log('🎯 Starting Tarsius app with Svelte...');

let app;

try {
  console.log('🔍 About to check target element...');

  // Check if target element exists
  const targetElement = document.getElementById('app');
  console.log('Target element found:', !!targetElement);

  if (!targetElement) {
    throw new Error('Target element #app not found');
  }

  console.log('🎯 Attempting Svelte app creation...');

  // Create and mount the app
  app = new App({
    target: targetElement,
    props: {
      name: 'world'
    }
  });

  console.log('✅ Tarsius Svelte app mounted successfully!');
  console.log('App target element content length:', targetElement.innerHTML.length);

  // Immediately show success indicator
  const successDiv = document.createElement('div');
  successDiv.style.cssText = 'position: fixed; top: 50px; left: 0; background: #FF9800; color: white; padding: 5px; font-family: Arial; z-index: 1000;';
  successDiv.textContent = 'Svelte Mounted ✓';
  document.body.appendChild(successDiv);

} catch (error) {
  console.error('❌ Error mounting Tarsius Svelte app:', error);

  // Show the working DOM version as fallback
  console.log('🔄 Falling back to basic DOM version...');

  const targetElement = document.getElementById('app');
  if (targetElement) {
    targetElement.innerHTML = `
      <main style="display: flex; height: 100vh; background: #f8f9fa;">
        <aside style="width: 200px; background: #f0f0f0; padding: 1rem; border-right: 1px solid #ddd;">
          <nav style="display: flex; flex-direction: column;">
            <button onclick="switchView('scratches')" style="margin-bottom: 0.5rem; padding: 0.5rem 1rem; border: 1px solid #ccc; background: white; cursor: pointer;">Scratches</button>
            <button onclick="switchView('projects')" style="margin-bottom: 0.5rem; padding: 0.5rem 1rem; border: 1px solid #ccc; background: white; cursor: pointer;">Projects</button>
            <button onclick="switchView('latex')" style="margin-bottom: 0.5rem; padding: 0.5rem 1rem; border: 1px solid #ccc; background: white; cursor: pointer;">LaTeX</button>
          </nav>
        </aside>
        <section id="content" style="flex: 1; padding: 1rem; background: white;">
          <div style="padding: 20px; background: #ffebee; border: 2px solid #f44336;">
            <h2>❌ Svelte Failed - Using Fallback</h2>
            <p><strong>Error:</strong> ${error.message}</p>
            <p>Basic DOM functionality works, but Svelte mounting failed.</p>
            <p>Navigation buttons above should still work.</p>
          </div>
        </section>
      </main>
    `;

    // Add global function for navigation
    window.switchView = function(view) {
      const content = document.getElementById('content');
      if (view === 'scratches') {
        content.innerHTML = `
          <div style="padding: 20px; background: #e8f5e8; border: 2px solid #4CAF50;">
            <h2>🗒️ Scratches View</h2>
            <p>Fallback DOM content - Svelte failed</p>
            <p>Current view: ${view}</p>
          </div>
        `;
      } else if (view === 'projects') {
        content.innerHTML = `
          <div style="padding: 20px; background: #e3f2fd; border: 2px solid #2196F3;">
            <h2>📁 Projects View</h2>
            <p>Fallback DOM content - Svelte failed</p>
            <p>Current view: ${view}</p>
          </div>
        `;
      } else if (view === 'latex') {
        content.innerHTML = `
          <h1>LaTeX</h1>
          <p>LaTeX view - fallback</p>
        `;
      }
    };
  }

  // Show error indicator
  const errorDiv = document.createElement('div');
  errorDiv.style.cssText = 'position: fixed; top: 75px; left: 0; background: #f44336; color: white; padding: 10px; font-family: Arial; z-index: 1000; max-width: 500px;';
  errorDiv.innerHTML = `<strong>Svelte Error:</strong><br><pre style="margin: 5px 0; font-size: 12px;">${error.message}</pre>`;
  document.body.appendChild(errorDiv);

  app = null;
}

export default app;