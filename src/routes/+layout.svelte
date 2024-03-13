<script lang="ts">
	import '../app.css';
	import {
		login,
		make_graph_request,
		refresh,
		check_login,
		currentUser,
		get_my_photo
	} from '$lib/graph';
	let user = currentUser;
	import { onMount } from 'svelte';
	let photo: string;
	onMount(async () => {
		await check_login();
		if ($user !== null) {
			photo = URL.createObjectURL(await get_my_photo($user));
		}
	});
</script>

<button on:click={login} class="text-red-400">Login</button>
<button on:click={make_graph_request} class="text-slate-200">Check Login</button>
<button on:click={refresh} class="text-green-400">Refresh</button>
<button
	on:click={async () => {
		photo = URL.createObjectURL(await get_my_photo($user));
	}}
	class="text-orange-400">Photo</button
>
<img src={photo} alt="" />
<slot />
