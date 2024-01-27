<script lang="ts">
	import type { Window } from '../../editor';
	import WebrayField from '../data/WebrayField.svelte';

	export let win: Window;
</script>

<div class="h-full flex flex-col gap-4">
	<div class="px-2 flex flex-col gap-4 snap-y snap-mandatory overflow-y-scroll scroll-smooth">
		{#each Array.from({ length: 30 }) as _, i}
			<WebrayField text={win.icon} />
		{/each}
	</div>
	
	<button type="button" class="btn variant-ghost-primary mx-2">Add</button>
</div>
