<script lang="ts">
	export let label: string;
	export let tooltip: string;
	export let initial: any;
	export let meta: any;
</script>

<span class="flex flex-row items-center justify-stretch gap-1">
	<p class="mr-1 text-surface-200 w-1/5">{label}</p>
	<input class="webray-input input text-center text-surface-300 w-4/5" type="text" placeholder="name" />
</span>
