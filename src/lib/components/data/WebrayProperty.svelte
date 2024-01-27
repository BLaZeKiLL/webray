<script lang="ts">
	import DataSelect from './inputs/data_select.svelte';
	import F32 from './inputs/f32.svelte';
	import Rgb from './inputs/rgb.svelte';
	import Str from './inputs/str.svelte';
	import U32 from './inputs/u32.svelte';
	import Vec3f from './inputs/vec3f.svelte';

	export let label: string;
	export let type: string;
	export let tooltip: string;
	export let initial: any;
	export let meta: any;

	let comp: any;

	$: switch (type) {
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
			console.log('beep');
			comp = DataSelect;
			break;
		default:
			console.error('Invalid input type: ' + type);
			break;
	}
</script>

<svelte:component this={comp} {label} {tooltip} {initial} {meta} />
