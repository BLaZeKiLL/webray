<script lang="ts">
	import { WebrayEditor, type WebrayToolbar } from '../../editor';
	import WebrayIcon from '../ui/WebrayIcon.svelte';

	export let toolbar: WebrayToolbar;

	const invoke = (action: string) => {
		WebrayEditor.invokeAction(action);
	};
</script>

<div class="flex flex-row">
	<div class="variant-ghost-surface btn-group rounded-none">
		{#each toolbar.lead as tool (tool.action)}
			<button class="!p-2" on:click={() => invoke(tool.action)}>
				<WebrayIcon icon={tool.icon} tooltip={tool.tooltip} />
			</button>
		{/each}
	</div>

	<span class="grow"></span>

	<div class="variant-ghost-surface btn-group rounded-none">
		{#each toolbar.center as tool (tool.action)}
			<button class="!p-2" on:click={() => invoke(tool.action)}>
				<WebrayIcon icon={tool.icon} tooltip={tool.tooltip} />
			</button>
		{/each}
	</div>

	<span class="grow"></span>

	<div class="variant-ghost-surface btn-group rounded-none">
		{#each toolbar.trail as tool (tool.action)}
			<button class="!p-2" on:click={() => invoke(tool.action)}>
				<WebrayIcon icon={tool.icon} tooltip={tool.tooltip} />
			</button>
		{/each}
	</div>
</div>
