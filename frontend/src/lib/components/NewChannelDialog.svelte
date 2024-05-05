<script lang="ts">
	import type { Channel } from '$lib/utils/types';
    import { base_endpoint } from '$lib/global';
	import { GradientButton, Modal, Button, Toggle, Label, Input } from 'flowbite-svelte';
	import { CirclePlusSolid } from 'flowbite-svelte-icons';

	let isDialogOpen: boolean = false;

	function newChannel() {
		const channel: Channel = {
			id: 0,
			title: '',
			description: '',
			image: '',
			active: true,
			url: '',
			max: 5,
			first: new Date(),
			created_at: new Date(),
			updated_at: new Date()
		};
		return channel;
	}

	function onChangeFirst(e: any) {
		console.log(e.target.valueAsDate);
	}

	function onOpendialogClicked() {
		isDialogOpen = true;
	}

	async function onNewChannelButtonClicked(e: any) {
		console.log(e);
		console.log(channel);
        console.log(nodeRef);
        return;
		const request = await fetch(`${base_endpoint}/api/1.0/channels/`, {
			method: 'POST',
			headers: {
				Accept: 'application/json',
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(channel)
		});
		const response = await request.json();
		console.log(response);
		if (response.status) {
			channel = response.data;
		}

		//channel = newChannel();
		//console.log(channel);
	}

	export let channel = newChannel();
	console.log(channel);
    let nodeRef: any;

	$: firstDate = channel.first.toISOString().split('T')[0];
</script>

<GradientButton on:click={onOpendialogClicked} bind:thist={nodeRef} class="mb-4">
	<CirclePlusSolid />
</GradientButton>

<Modal bind:open={isDialogOpen} autoclose outsideclose size="xs" class="w-full">
	<form class="flex flex-col space-y-6" action="">
		<h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">New Channel</h3>
		<!--
        active: boolean
        url: string
        first: date
        max: number
        -->
		<Toggle bind:checked={channel.active}>Active</Toggle>
		<Label class="space-y-2">
			<span>Url</span>
			<Input type="url" name="url" placeholder="url" bind:value={channel.url} required />
		</Label>
		<Label class="space-y-2">
			<span>Max number of episodes</span>
			<Input
				type="number"
				name="max"
				placeholder="max"
				min="-1"
				bind:value={channel.max}
				required
			/>
		</Label>
		<Label class="space-y-2">
			<span>First episode date</span>
			<Input
				type="date"
				name="first"
				placeholder="first"
				bind:value={firstDate}
				on:change={onChangeFirst}
				on:input={(e) => {
					if (e.target != null && e.target.value != null) {
						channel.first = new Date(e.target.value);
					}
				}}
				required
			/>
		</Label>
		<Button on:click={onNewChannelButtonClicked}>Create channel</Button>
	</form>
</Modal>
