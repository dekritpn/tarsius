import App from './App.svelte';

console.log('Starting Tarsius app...');

let app;

try {
  app = new App({
    target: document.body,
    props: {
      name: 'world'
    }
  });

  console.log('Tarsius app mounted successfully');
} catch (error) {
  console.error('Error mounting Tarsius app:', error);
  // Fallback: show error message
  document.body.innerHTML = `
    <div style="padding: 20px; font-family: Arial, sans-serif;">
      <h1>Error Loading Tarsius</h1>
      <p>There was an error starting the application:</p>
      <pre style="background: #f5f5f5; padding: 10px; border-radius: 4px;">${error.message}</pre>
    </div>
  `;
}

export default app;