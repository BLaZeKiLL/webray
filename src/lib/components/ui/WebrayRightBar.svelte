<script lang="ts">
	import WebrayField from "../fields/WebrayField.svelte";
    import WebrayDrawer from "../drawer/WebrayDrawer.svelte";

    import Camera from '~icons/material-symbols/android-camera-outline';
    import RenderSettings from '~icons/cil/tv';
</script>

<div class="h-full flex flex-col border-l border-surface-600">
    <div class="h-1/2">
        <WebrayDrawer>
            <h2 slot="header">
                <Camera style="font-size: 1.5rem"/>
            </h2>

            {#each Array.from({ length: 10 }) as _, i}
                <WebrayField />
            {/each}
        </WebrayDrawer>
    </div>
    <hr>
    <div class="h-1/2">
        <WebrayDrawer>
            <h2 slot="header">
                <RenderSettings style="font-size: 1.5rem"/>
            </h2>

            {#each Array.from({ length: 10 }) as _, i}
                <WebrayField /> 
            {/each}
        </WebrayDrawer>
    </div>
</div>