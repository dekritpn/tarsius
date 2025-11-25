// Test basic mounting without Svelte
console.log('🎯 Starting Tarsius app - testing basic DOM manipulation...');

try {
  // Check if target element exists
  const targetElement = document.getElementById('app');
  console.log('Target element found:', !!targetElement);

  if (!targetElement) {
    throw new Error('Target element #app not found');
  }

  console.log('🎯 Testing basic DOM manipulation...');

  // Create basic content without Svelte
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
        <div style="padding: 20px; background: #e8f5e8; border: 2px solid #4CAF50;">
          <h2>🗒️ Scratches View</h2>
          <p>Basic DOM content - no Svelte</p>
          <p>Current view: scratches</p>
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
          <p>Basic DOM content - no Svelte</p>
          <p>Current view: ${view}</p>
        </div>
      `;
    } else if (view === 'projects') {
      content.innerHTML = `
        <div style="padding: 20px; background: #e3f2fd; border: 2px solid #2196F3;">
          <h2>📁 Projects View</h2>
          <p>Basic DOM content - no Svelte</p>
          <p>Current view: ${view}</p>
        </div>
      `;
    } else if (view === 'latex') {
      content.innerHTML = `
        <h1>LaTeX</h1>
        <p>LaTeX view - no Svelte</p>
      `;
    }
  };

  console.log('✅ Basic DOM manipulation successful');
  console.log('App target element content:', targetElement.innerHTML);

  // Immediately show success indicator
  const successDiv = document.createElement('div');
  successDiv.style.cssText = 'position: fixed; top: 50px; left: 0; background: #FF9800; color: white; padding: 5px; font-family: Arial; z-index: 1000;';
  successDiv.textContent = 'DOM Mounted ✓ (No Svelte)';
  document.body.appendChild(successDiv);

} catch (error) {
  console.error('❌ Error with basic DOM manipulation:', error);

  // Immediately show error indicator
  const errorDiv = document.createElement('div');
  errorDiv.style.cssText = 'position: fixed; top: 50px; left: 0; background: #f44336; color: white; padding: 10px; font-family: Arial; z-index: 1000; max-width: 500px;';
  errorDiv.innerHTML = `<strong>DOM Failed ✗</strong><br><pre style="margin: 5px 0; font-size: 12px;">${error.message}</pre><br><strong>Stack:</strong><br><pre style="margin: 5px 0; font-size: 10px; max-height: 200px; overflow: auto;">${error.stack}</pre>`;
  document.body.appendChild(errorDiv);
}