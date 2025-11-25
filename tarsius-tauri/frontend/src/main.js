import App from './App.svelte';

console.log('🎯 Starting Tarsius app...');

let app;

try {
  // Check if target element exists
  const targetElement = document.getElementById('app');
  console.log('Target element found:', !!targetElement);

  if (!targetElement) {
    throw new Error('Target element #app not found');
  }

  // Create and mount the app
  app = new App({
    target: targetElement,
    props: {
      name: 'world'
    }
  });

  console.log('✅ Tarsius app mounted successfully');
  console.log('App target element content:', targetElement.innerHTML);
} catch (error) {
  console.error('❌ Error mounting Tarsius app:', error);
  app = null;

  // Fallback: show error in the page
  const errorDiv = document.createElement('div');
  errorDiv.style.cssText = 'position: fixed; top: 75px; left: 0; background: #f44336; color: white; padding: 10px; font-family: Arial; z-index: 1000; max-width: 400px;';
  errorDiv.innerHTML = `<strong>Svelte Error:</strong><br>${error.message}`;
  document.body.appendChild(errorDiv);
}

export default app;