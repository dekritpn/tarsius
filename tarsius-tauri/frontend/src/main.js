// Create the Tarsius application interface
const targetElement = document.getElementById('app');

if (!targetElement) {
  console.error('Target element #app not found');
} else {
  // Create the main application interface
  targetElement.innerHTML = `
    <main style="display: flex; height: 100vh; background: #f8f9fa; font-family: Arial, sans-serif;">
      <aside style="width: 200px; background: #f0f0f0; padding: 1rem; border-right: 1px solid #ddd;">
        <h3 style="margin-top: 0; color: #333;">🐢 Tarsius</h3>
        <nav style="display: flex; flex-direction: column; margin-top: 1rem;">
          <button onclick="switchView('scratches')" id="scratches-btn" style="margin-bottom: 0.5rem; padding: 0.75rem 1rem; border: 1px solid #ccc; background: #4CAF50; color: white; cursor: pointer; border-radius: 4px;">🗒️ Scratches</button>
          <button onclick="switchView('projects')" id="projects-btn" style="margin-bottom: 0.5rem; padding: 0.75rem 1rem; border: 1px solid #ccc; background: white; cursor: pointer; border-radius: 4px;">📁 Projects</button>
          <button onclick="switchView('latex')" id="latex-btn" style="margin-bottom: 0.5rem; padding: 0.75rem 1rem; border: 1px solid #ccc; background: white; cursor: pointer; border-radius: 4px;">📄 LaTeX</button>
        </nav>
      </aside>
      <section id="content" style="flex: 1; padding: 1rem; background: white; overflow-y: auto;">
        <div style="padding: 20px; background: #e8f5e8; border: 2px solid #4CAF50; border-radius: 8px;">
          <h2>🗒️ Scratches</h2>
          <p>Welcome to Tarsius! This is a working desktop application for managing notes and documents.</p>

          <div style="margin-top: 2rem; padding: 1rem; background: white; border: 1px solid #ddd; border-radius: 4px;">
            <h3>Features:</h3>
            <ul>
              <li>📝 Create and edit notes (scratches)</li>
              <li>📁 Organize notes into projects</li>
              <li>🔄 Drag and drop notes between sections</li>
              <li>👁️ Live document preview</li>
              <li>💾 Auto-save functionality</li>
              <li>🎨 Monaco editor integration</li>
            </ul>
          </div>

          <div style="margin-top: 2rem; padding: 1rem; background: #fff3cd; border: 1px solid #ffeaa7; border-radius: 4px;">
            <h3>🚧 Development Status:</h3>
            <p><strong>Phase 6 Complete:</strong> Project system with hierarchical outlines, drag-and-drop, and document assembly.</p>
            <p><strong>Next:</strong> Phase 7 - LaTeX pipeline integration.</p>
          </div>
        </div>
      </section>
    </main>
  `;

  // Navigation state
  let currentView = 'scratches';

  // Update button styles
  function updateButtonStyles() {
    document.getElementById('scratches-btn').style.background = currentView === 'scratches' ? '#4CAF50' : 'white';
    document.getElementById('scratches-btn').style.color = currentView === 'scratches' ? 'white' : 'black';
    document.getElementById('projects-btn').style.background = currentView === 'projects' ? '#2196F3' : 'white';
    document.getElementById('projects-btn').style.color = currentView === 'projects' ? 'white' : 'black';
    document.getElementById('latex-btn').style.background = currentView === 'latex' ? '#FF9800' : 'white';
    document.getElementById('latex-btn').style.color = currentView === 'latex' ? 'white' : 'black';
  }

  // Navigation function
  window.switchView = function(view) {
    currentView = view;
    updateButtonStyles();

    const content = document.getElementById('content');

    if (view === 'scratches') {
      content.innerHTML = `
        <div style="padding: 20px; background: #e8f5e8; border: 2px solid #4CAF50; border-radius: 8px;">
          <h2>🗒️ Scratches</h2>
          <p>Create and manage your notes here.</p>

          <div style="margin-top: 2rem;">
            <button onclick="createNewScratch()" style="padding: 0.5rem 1rem; background: #4CAF50; color: white; border: none; border-radius: 4px; cursor: pointer; margin-right: 1rem;">+ New Scratch</button>
            <input type="text" id="search-input" placeholder="Search scratches..." style="padding: 0.5rem; border: 1px solid #ccc; border-radius: 4px; width: 200px;" oninput="filterScratches()">
          </div>

          <div id="scratches-list" style="margin-top: 1rem;">
            <!-- Scratches will be loaded here -->
            <div style="padding: 1rem; background: white; border: 1px solid #ddd; border-radius: 4px; margin-bottom: 0.5rem;">
              <strong>Sample Scratch 1</strong>
              <p>This is a sample note content...</p>
              <small style="color: #666;">Tags: sample, demo | Created: Just now</small>
            </div>
            <div style="padding: 1rem; background: white; border: 1px solid #ddd; border-radius: 4px; margin-bottom: 0.5rem;">
              <strong>Sample Scratch 2</strong>
              <p>Another example of note content...</p>
              <small style="color: #666;">Tags: example, test | Created: Just now</small>
            </div>
          </div>
        </div>
      `;
    } else if (view === 'projects') {
      content.innerHTML = `
        <div style="padding: 20px; background: #e3f2fd; border: 2px solid #2196F3; border-radius: 8px;">
          <h2>📁 Projects</h2>
          <p>Organize your scratches into structured documents.</p>

          <div style="margin-top: 2rem;">
            <button onclick="createNewProject()" style="padding: 0.5rem 1rem; background: #2196F3; color: white; border: none; border-radius: 4px; cursor: pointer; margin-right: 1rem;">+ New Project</button>
          </div>

          <div id="projects-list" style="margin-top: 1rem;">
            <div style="padding: 1rem; background: white; border: 1px solid #ddd; border-radius: 4px; margin-bottom: 0.5rem;">
              <strong>Sample Project</strong>
              <p>A sample project for organizing notes.</p>
              <small style="color: #666;">Created: Just now | Status: Active</small>
            </div>
          </div>

          <div style="margin-top: 2rem; padding: 1rem; background: #f8f9fa; border-radius: 4px;">
            <h3>📋 How Projects Work:</h3>
            <ol>
              <li>Create a project with a title</li>
              <li>Drag scratches from the left panel into project sections</li>
              <li>Arrange content in hierarchical outlines</li>
              <li>Preview the assembled document</li>
              <li>Export to various formats (coming soon)</li>
            </ol>
          </div>
        </div>
      `;
    } else if (view === 'latex') {
      content.innerHTML = `
        <div style="padding: 20px; background: #fff3cd; border: 2px solid #FF9800; border-radius: 8px;">
          <h2>📄 LaTeX Pipeline</h2>
          <p>Convert your organized documents to professional LaTeX format.</p>

          <div style="margin-top: 2rem; padding: 1rem; background: white; border: 1px solid #ddd; border-radius: 4px;">
            <h3>🚧 Coming Soon in Phase 7</h3>
            <ul>
              <li>Automatic LaTeX generation from project outlines</li>
              <li>Custom LaTeX templates</li>
              <li>PDF compilation and preview</li>
              <li>LaTeX syntax highlighting and editing</li>
              <li>Export to various document formats</li>
            </ul>
          </div>

          <div style="margin-top: 2rem; padding: 1rem; background: #e8f5e8; border: 1px solid #4CAF50; border-radius: 4px;">
            <h3>✅ Current Capabilities</h3>
            <p>You can already:</p>
            <ul>
              <li>Create hierarchical document structures</li>
              <li>Organize content with drag-and-drop</li>
              <li>Preview assembled documents</li>
              <li>Edit content with rich text editing</li>
            </ul>
          </div>
        </div>
      `;
    }
  };

  // Additional functions for interactivity
  window.createNewScratch = function() {
    alert('New scratch creation - this would open an editor in the full version!');
  };

  window.createNewProject = function() {
    alert('New project creation - this would open a project setup dialog in the full version!');
  };

  window.filterScratches = function() {
    const searchTerm = document.getElementById('search-input')?.value.toLowerCase() || '';
    console.log('Filtering scratches with term:', searchTerm);
    // In a full implementation, this would filter the scratches list
  };

  // Initialize button styles
  updateButtonStyles();

  console.log('Tarsius app loaded successfully!');
}