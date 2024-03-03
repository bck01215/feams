<script lang="ts">
	import ChatNav from '$lib/ui/Chat/ChatNav.svelte';
</script>

<div class="h-screen bg-zinc-900">
	<ChatNav />
</div>

<style lang="postcss">
	:global(html) {
		background-color: theme(colors.zinc.900);
	}
</style>
