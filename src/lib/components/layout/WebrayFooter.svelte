<script lang="ts">
	import { AppBar } from '@skeletonlabs/skeleton';
	import MdiGithub from '~icons/mdi/github';
	import MdiYoutube from '~icons/mdi/youtube';
	import MdiLinkedin from '~icons/mdi/linkedin';
	import MdiInstagram from '~icons/mdi/instagram';
	import MaterialSymbolsInfo from '~icons/material-symbols/info';
</script>

<div class="flex h-full w-full flex-col border-t border-surface-600">
	<span class="grow"></span>
	<AppBar gridColumns="grid-cols-3" slotDefault="place-self-center" slotTrail="place-content-end">
		<svelte:fragment slot="lead">
			<span class="flex flex-row items-center gap-1 font-bold">
				<h3 class="h3">&lbrace;</h3>
				<h4 class="cb-c h4">C</h4>
				<h4 class="cb-b h4">B</h4>
				<h3 class="h3">&rbrace;</h3>
			</span>
		</svelte:fragment>

		<h3 class="h3 font-extrabold">WebRay</h3>

		<svelte:fragment slot="trail">
			<a
				href="https://github.com/BLaZeKiLL/webray?tab=readme-ov-file#editor-documentation"
				target="_blank"
				rel="noopener noreferrer"
				class="btn-icon hover:variant-soft-primary"
			>
				<MaterialSymbolsInfo style="font-size: 1.5rem" />
			</a>
			<a
				href="https://www.linkedin.com/in/devashish-lal-096868176/"
				target="_blank"
				rel="noopener noreferrer"
				class="btn-icon hover:variant-soft-primary"
			>
				<MdiLinkedin style="font-size: 1.5rem" />
			</a>
			<a
				href="https://www.youtube.com/channel/UC_qfPIYfXOvg0SDAc8Z68WA"
				target="_blank"
				rel="noopener noreferrer"
				class="btn-icon hover:variant-soft-primary"
			>
				<MdiYoutube style="font-size: 1.5rem" />
			</a>
			<a
				href="https://www.instagram.com/code.blaze/?igshid=MmIzYWVlNDQ5Yg%3D%3D"
				target="_blank"
				rel="noopener noreferrer"
				class="btn-icon hover:variant-soft-primary"
			>
				<MdiInstagram style="font-size: 1.5rem" />
			</a>
			<a
				href="https://github.com/BLaZeKiLL"
				target="_blank"
				rel="noopener noreferrer"
				class="btn-icon hover:variant-soft-primary"
			>
				<MdiGithub style="font-size: 1.5rem" />
			</a>
		</svelte:fragment>
	</AppBar>
</div>

<style>
	.cb-c {
		color: #c02a01;
	}

	.cb-b {
		color: #f8a905;
	}
</style>
