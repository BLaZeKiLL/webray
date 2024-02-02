<script lang="ts">
	import WebrayDataView from './WebrayDataView.svelte';
	import scene from '../../store/scene.store';
	import { get_prop } from '../../utils/object.extensions';

	export let data_type: string;
	export let bind_path: string;

	$: count = get_prop(scene.current, bind_path.split(':')[2]).length;
</script>

{#each { length: count } as _, i (i)}
	<WebrayDataView {data_type} bind_path={`${bind_path}[${i}]`} />
{/each}
