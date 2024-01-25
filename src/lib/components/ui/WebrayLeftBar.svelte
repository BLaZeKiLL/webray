<script lang="ts">
	import WebrayField from "../fields/WebrayField.svelte";
    import WebrayDrawer from "../drawer/WebrayDrawer.svelte";

    import Scene from "~icons/iconamoon/3d-light";
</script>

<div class="h-full flex flex-col border-r border-surface-600">
    <WebrayDrawer>
        <span slot="header" class="flex flex-row">
            <Scene style="font-size: 1.5rem"/>
        </span>

        {#each Array.from({ length: 30 }) as _, i}
            <WebrayField />
        {/each}

        <span slot="footer">
            +
        </span>
    </WebrayDrawer>
</div>