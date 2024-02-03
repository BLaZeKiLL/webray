<script lang="ts">
	import WebrayDataView from './WebrayDataView.svelte';
	import scene from '../../store/scene.store';
	import { get_prop } from '../../utils/object.extensions';
	import WebrayIcon from '../ui/WebrayIcon.svelte';

	export let data_type: string;
	export let bind_path: string;

	$: count = get_prop(scene.current, bind_path.split(':')[2]).length;
</script>

{#each { length: count } as _, i (i)}
	<WebrayDataView {data_type} bind_path={`${bind_path}[${i}]`}>
		<span slot="header" class="flex flex-row items-center">
			<h4 class="h4 text-surface-200">ID: {i}</h4>
			<span class="grow"></span>
			<button>
				<WebrayIcon icon="i_delete_item" tooltip="Delete item" css="text-error-400" />
			</button>
		</span>
	</WebrayDataView>
{/each}
