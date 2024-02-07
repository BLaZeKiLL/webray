<script lang="ts">
	import { AppBar } from '@skeletonlabs/skeleton';
	import MdiGithub from '~icons/mdi/github';
	import MdiYoutube from '~icons/mdi/youtube';
	import MdiLinkedin from '~icons/mdi/linkedin';
	import MdiInstagram from '~icons/mdi/instagram';
</script>

<div class="flex h-full w-full flex-col border-t border-surface-600">
	<span class="grow"></span>
	<AppBar>
		<svelte:fragment slot="lead">
			<h3 class="h3">WebRay</h3>
		</svelte:fragment>

		<svelte:fragment slot="trail">
			<a
				href="https://www.linkedin.com/in/devashish-lal-096868176/"
				class="btn-icon hover:variant-soft-primary"
			>
				<MdiLinkedin style="font-size: 1.5rem" />
			</a>
			<a
				href="https://www.youtube.com/channel/UC_qfPIYfXOvg0SDAc8Z68WA"
				class="btn-icon hover:variant-soft-primary"
			>
				<MdiYoutube style="font-size: 1.5rem" />
			</a>
			<a
				href="https://www.instagram.com/code.blaze/?igshid=MmIzYWVlNDQ5Yg%3D%3D"
				class="btn-icon hover:variant-soft-primary"
			>
				<MdiInstagram style="font-size: 1.5rem" />
			</a>
			<a href="https://github.com/BLaZeKiLL/webray" class="btn-icon hover:variant-soft-primary">
				<MdiGithub style="font-size: 1.5rem" />
			</a>
		</svelte:fragment>
	</AppBar>
</div>
