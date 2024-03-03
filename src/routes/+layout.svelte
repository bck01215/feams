<script>
	import { onMount } from 'svelte';
	import '../app.css';
	import { invoke } from '@tauri-apps/api/tauri';

	async function login() {
		await invoke('authenticate');
	}
</script>

<button on:click={login}>Login</button>
<slot />
