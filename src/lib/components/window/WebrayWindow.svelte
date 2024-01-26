<script lang="ts">
	import type { Window } from "../../editor";
	import WebrayField from "../fields/WebrayField.svelte";

    export let win: Window;
</script>

{#each Array.from({ length: 30 }) as _, i}
    <WebrayField text={win.icon}/>
{/each}