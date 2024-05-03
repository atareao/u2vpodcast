<script lang="ts">

	import type { PageData, ActionData } from './$types';
    import NewChannelDialog from '$lib/components/NewChannelDialog.svelte';
    import ChannelCardEditor from '$lib/components/ChannelCardEditor.svelte';
    import { GradientButton, Modal, Button, Toggle, Label, Input } from 'flowbite-svelte';
    import { CirclePlusSolid } from 'flowbite-svelte-icons';
    import type { Channel } from '$lib/utils/types';
    export const showConfirmDialog: boolean = false;
	export let newForm: ActionData;
    let isFormOpen: boolean;

	export let data: PageData;

	let channels: Channel[] = data.channels;

    function onNewChannelClicked(){
        isFormOpen = true;
    }
</script>

<div class="grid justify-items-center">
    {#each channels as channel}
        <ChannelCardEditor {channel} />
    {/each}

    <NewChannelDialog/>
</div>

