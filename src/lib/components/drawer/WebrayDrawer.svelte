<script lang="ts">
</script>

<div class="flex max-h-full flex-col py-2">
	<span class="px-2">
		<slot name="header" />
	</span>

	<div
		class="my-2 flex snap-y snap-mandatory scroll-py-4 flex-col gap-4 overflow-y-scroll scroll-smooth px-2"
	>
		<slot />
	</div>

	<span class="px-2">
		<slot name="footer" />
	</span>
</div>
