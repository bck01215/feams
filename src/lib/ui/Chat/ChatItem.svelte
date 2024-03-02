<script>
	import { openChat } from '$lib/stores';
	export let title = '';
	export let description = '';
</script>

<div
	on:click={() => openChat.set(title)}
	on:keydown={(event) => {
		if (event.key === 'Enter') {
			openChat.set(title);
		}
	}}
	role="button"
	tabindex="0"
	class="h-10 overflow-hidden {$openChat === title
		? 'text-white'
		: 'text-zinc-600'} hover:bg-zinc-900 hover:text-white
   pl-1 border-teams-blue-200 border-b"
>
	<h2 class="text-sm font-bold line-clamp-1">
		{title}
	</h2>
	<p class="text-xs">
		{description}
	</p>
</div>
