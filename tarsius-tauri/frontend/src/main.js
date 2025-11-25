import App from './App.svelte';

console.log('🎯 Starting Tarsius app...');

// Simple mount without error handling
const app = new App({
  target: document.body,
  props: {
    name: 'world'
  }
});

console.log('✅ Tarsius app mounted successfully');

export default app;