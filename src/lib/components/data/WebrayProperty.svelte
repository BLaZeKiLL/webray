<script lang="ts">
	import type { WebrayProperty } from '../../editor';
	import DataSelect from './inputs/data_select.svelte';
	import F32 from './inputs/f32.svelte';
	import Rgb from './inputs/rgb.svelte';
	import Str from './inputs/str.svelte';
	import U32 from './inputs/u32.svelte';
	import Vec3f from './inputs/vec3f.svelte';

	export let property: WebrayProperty;
	export let bind_path: string;

	let comp: any;

	$: switch (property.type) {
		case 'f32':
			comp = F32;
			break;
		case 'rgb':
			comp = Rgb;
			break;
		case 'u32':
			comp = U32;
			break;
		case 'vec3f':
			comp = Vec3f;
			break;
		case 'str':
			comp = Str;
			break;
		case 'data_select':
			comp = DataSelect;
			break;
		default:
			console.error('Invalid input type: ' + property.type);
			break;
	}
</script>

<svelte:component this={comp} {property} {bind_path}/>
