
<script lang="ts">

	import type { PageData, ActionData } from './$types';
    import ChannelCardEditor from '$lib/components/ChannelCardEditor.svelte';
    import { GradientButton, Modal, Button, Toggle, Label, Input } from 'flowbite-svelte';
    import { CirclePlusSolid } from 'flowbite-svelte-icons';
    import type { Channel } from '$lib/utils/types';
    export const showConfirmDialog: boolean = false;
	export let newForm: ActionData;
    let isNewFormOpen: boolean;

	export let data: PageData;

	let channels: Channel[] = data.channels;

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
        url: string
        first: date
        max: number
        -->
		<Toggle>Active</Toggle>
		<Label class="space-y-2">
			<span>Url</span>
			<Input
				type="url"
				name="url"
				placeholder="url"
				required
			/>
		</Label>
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

