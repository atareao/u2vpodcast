<script lang="ts">
	import { onMount } from 'svelte';
	import type { PageData, ActionData } from './$types';
    import { type Channel, } from '$lib/channel';
    import ChannelCardEditor from '$lib/components/ChannelCardEditor.svelte';
    import { base_endpoint } from '$lib/global';
    import { GradientButton, Modal, Button, Toggle, Label, Input } from 'flowbite-svelte';
    import { CirclePlusSolid } from 'flowbite-svelte-icons';
	const endpoint = `${base_endpoint}/api/1.0/channels/`;
    export let showConfirmDialog: boolean;
	export let newData: PageData;
	export let newForm: ActionData;
    let isNewFormOpen: boolean;
	let channels: Channel[] = [];

    async function getChannels(): Promise<Channel[]>{
        const request = await fetch(endpoint);
        const data = await request.json();
        console.log("======");
        console.log(data);
        return data;
    }

	onMount(async function () {
        channels = await getChannels();
	});
	const endPoint = `${base_endpoint}/api/1.0/channels/`;

    function onNewChannelClicked(){
        isNewFormOpen = true;
    }
</script>

<div class="grid justify-items-center">
    <GradientButton
        on:click={onNewChannelClicked}
        class="mb-4">
        <CirclePlusSolid/>
    </GradientButton>
    {#each channels as channel}
        <ChannelCardEditor {channel} />
    {/each}
</div>

<Modal bind:open={isNewFormOpen} autoclose outsideclose size="xs" class="w-full">
	<newForm class="flex flex-col space-y-6">
		{#if newForm?.missing}<p class="error">The email field is required</p>{/if}
		<h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">
			New Channel
		</h3>
		<!--
        active: boolean
        first: date
        max: number
        -->
		<Toggle>Active</Toggle>
		<Label class="space-y-2">
			<span>Max number of episodes</span>
			<Input
				type="number"
				name="max"
				placeholder="max"
				min="1"
				required
			/>
		</Label>
		<Label class="space-y-2">
			<span>First episode date</span>
			<Input
				type="date"
				name="first"
				placeholder="first"
				required
			/>
		</Label>
		<Button>
            Create channel
        </Button>
	</newForm>
</Modal>
