<script lang="ts">
	import F32 from './inputs/f32.svelte';
	import Rgb from './inputs/rgb.svelte';
	import String from './inputs/string.svelte';
	import U32 from './inputs/u32.svelte';
	import Vec3f from './inputs/vec3f.svelte';

	export let text: string;
</script>

<div class="card flex snap-start flex-col gap-2 p-2">
	<String label={text} />
	<Vec3f label={text} />
	<Rgb label={text} />
	<F32 label={text} />
	<U32 label={text} />
</div>
