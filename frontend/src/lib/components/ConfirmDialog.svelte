<script lang="ts">
	import { Button, Modal} from 'flowbite-svelte';
    import {CheckOutline, CloseOutline} from 'flowbite-svelte-icons';
    export let open: boolean;
    export let okFunction: any;
    export let title: string;
    export let message: string;

    function onClickOk(){
        open = false;
        okFunction();
    }
    function onClickCancel(){
        open = false;
    }
</script>

<Modal bind:open size="xs">
    <div>
        {#if title && title != ""}
        <h3><strong>{title}</strong></h3>
        {/if}
        <span>{message}</span>
    </div>
    <div class="block space-y-4 md:space-y-0 md:space-x-4">
        <Button on:click={onClickOk}><CheckOutline/></Button>
        <Button on:click={onClickCancel}><CloseOutline/></Button>
    </div>
</Modal>
