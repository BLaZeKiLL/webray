<script lang="ts">
	import WebrayDataView from './WebrayDataView.svelte';
	import scene from '../../store/scene.store';
	import WebrayIcon from '../ui/WebrayIcon.svelte';
	import { ID, WebrayEditor } from '../../editor';

	export let data_type: string;
	export let bind_path: string;

	const store = scene.derived<any[]>(bind_path);
</script>

{#each $store as _, i (i)}
	<WebrayDataView {data_type} bind_path={`${bind_path}[${i}]`}>
		<span slot="header" class="flex flex-row items-center">
			<h4 class="h4 text-surface-200">ID: {i}</h4>
			<span class="grow"></span>
			<button on:click={() => WebrayEditor.invokeAction(ID.a_del_list_item, { bind_path: `${bind_path}[${i}]` })}>
				<WebrayIcon icon="i_delete_item" tooltip="Delete item" css="text-error-400" />
			</button>
		</span>
	</WebrayDataView>
{/each}
