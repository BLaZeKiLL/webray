<script lang="ts">
	import WebrayField from "../fields/WebrayField.svelte";
import WebrayWindow from "../windows/WebrayWindow.svelte";
</script>

<div class="h-full flex flex-col border-r border-surface-600">
    <div class="h-full">
        <WebrayWindow>
            <span slot="header" class="sticky flex flex-row">
                <h2>Scene</h2>
                <span class="grow"></span>
                <span>+</span>
            </span>

            {#each Array.from({ length: 30 }) as _, i}
                <WebrayField />
            {/each}
        </WebrayWindow>
    </div>
</div>