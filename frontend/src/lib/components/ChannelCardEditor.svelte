<script lang="ts">
	import { base_endpoint } from '$lib/global';
	import type { PageData, ActionData } from './$types';
    import ConfirmDialog from './ConfirmDialog.svelte';

	let isFormOpen: boolean;
    let showConfirmDialog: boolean;
	export let data: PageData;
	export let form: ActionData;
	import { GradientButton, Modal, Button, Label, Input, Toggle } from 'flowbite-svelte';
	import { EditSolid, TrashBinSolid } from 'flowbite-svelte-icons';
	import { type Channel } from '$lib/channel';
	let nodeRef: any;
	export let channel: Channel;

	const endPoint = `${base_endpoint}/api/1.0/channels/`;
	function deleteChannel() {
        console.log("deleteChannel");
		nodeRef.parentNode.removeChild(nodeRef);
        console.log(nodeRef);
        console.log("deleted");
	}
	function editChannel() {
		isFormOpen = true;
	}
	function onChangeMax() {
		if (channel.max <= 0) {
			channel.max = 1;
		}
	}
	async function onButtonUpdateClicked() {
		const data = {
			id: channel.id,
			url: channel.url,
			active: channel.active,
			first: new Date(channel.first),
			max: parseInt(channel.max)
		};
		console.log(data);
		const config = {
			method: 'PUT',
			headers: {
				Accept: 'application/json',
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(data)
		};
		const request = await fetch(endPoint, config);
		const response = await request.json();
		console.log(response);
		if (response.status) {
			channel = response.data;
		}
	}
	function onChangeFirst(e: any) {
		console.log(e);
	}
	const inactiveClass =
		'bg-red-500 border-red-500 hover:bg-red-700 dark:border-red-700 dark:bg-red-800 dark:hover:bg-red-700';
	const activeClass =
		'bg-white border-gray-200 hover:bg-gray-100 dark:border-gray-700 dark:bg-gray-800 dark:hover:bg-gray-700';
	$: channel.firstDate = channel.first.split('T')[0];
</script>

<div>
	<div
		class="flex flex-col items-center {channel.active
			? activeClass
			: inactiveClass} border rounded-lg shadow md:flex-row md:max-w-xl"
		bind:this={nodeRef}
	>
		<img
			class="object-cover w-full rounded-t-lg h-96 md:h-auto md:w-48 md:rounded-none md:rounded-s-lg"
			alt={channel.title}
			src={channel.image}
		/>
		<div class="flex flex-col justify-between p-4 leading-normal">
			<h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
				{channel.title}
			</h5>
			<p class="mb-3 font-normal text-center">{channel.description}</p>
			<GradientButton class="mb-2" color="cyanToBlue" on:click={editChannel} pill>
				<EditSolid class="w-6 h-6" />
			</GradientButton>
			<GradientButton color="pinkToOrange" on:click={()=>showConfirmDialog=true} pill>
				<TrashBinSolid class="w-6 h-6" />
			</GradientButton>
		</div>
	</div>
</div>

<Modal bind:open={isFormOpen} autoclose outsideclose size="xs" class="w-full">
	<form class="flex flex-col space-y-6" action={endPoint} method="POST">
		{#if form?.missing}<p class="error">The email field is required</p>{/if}
		<h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">
			Edit <spam class="text-red-600">{channel.title}</spam> Channel
		</h3>
		<!--
        active: boolean
        first: date
        max: number
        -->
		<Toggle bind:checked={channel.active}>Active</Toggle>
		<Label class="space-y-2">
			<span>Max number of episodes</span>
			<Input
				type="number"
				name="max"
				placeholder="max"
				min="1"
				bind:value={channel.max}
				required
				on:change={onChangeMax}
			/>
		</Label>
		<Label class="space-y-2">
			<span>First episode date</span>
			<Input
				type="date"
				name="first"
				placeholder="first"
				bind:value={channel.firstDate}
				on:change={onChangeFirst}
				on:input={(e) => (channel.first = e.target.value)}
				required
			/>
		</Label>
		<Button on:click={onButtonUpdateClicked} type="submit">Update channel</Button>
	</form>
</Modal>

<ConfirmDialog
    bind:open={showConfirmDialog}
    title="Warning"
    message="Are you sure?"
    okFunction={deleteChannel}
    on:close={() => console.log('closed')}>
	Are you sure?
</ConfirmDialog>
