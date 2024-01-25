<script lang="ts">
	import WebrayField from "../fields/WebrayField.svelte";
import WebrayWindow from "../windows/WebrayWindow.svelte";

</script>

<div class="h-full flex flex-col border-l border-surface-600">
    <div class="h-1/2">
        <WebrayWindow>
            <h2 slot="header">Camera</h2>

            {#each Array.from({ length: 10 }) as _, i}
                <WebrayField />
            {/each}
        </WebrayWindow>
    </div>
    <hr>
    <div class="h-1/2">
        <WebrayWindow>
            <h2 slot="header">Render Settings</h2>

            {#each Array.from({ length: 10 }) as _, i}
                <WebrayField /> 
            {/each}
        </WebrayWindow>
    </div>
</div>