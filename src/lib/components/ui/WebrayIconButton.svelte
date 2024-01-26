<script lang="ts">
	import { popup, type PopupSettings } from "@skeletonlabs/skeleton";
	import { Icons } from "../../editor";

    export let icon;
    export let tooltip: string;

    const _icon = Icons.getIcon(icon);
    const target = `tooltip_${icon}`;

    const tooltipPopup: PopupSettings = {
        event: 'hover',
        placement: 'bottom',
        target
    };
</script>

<button type="button" class="btn-icon !bg-transparent" use:popup={tooltipPopup}>
    <svelte:component this={_icon} style="font-size: 1.5rem"/>
</button>

<div class="card p-1 variant-filled-secondary" data-popup={target}>
    {tooltip}
</div>