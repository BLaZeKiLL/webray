<script lang="ts">
	export let label: string;
</script>

<span class="flex flex-row items-center justify-stretch gap-1">
	<p class="mr-1 text-surface-200">{label}</p>
	<input class="webray-input input text-center text-surface-300" type="number" placeholder="X" />
	<input class="webray-input input text-center text-surface-300" type="number" placeholder="Y" />
	<input class="webray-input input text-center text-surface-300" type="number" placeholder="Z" />
</span>
