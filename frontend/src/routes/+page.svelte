<script lang="ts">
	import type { PageData } from './$types';
	import ConfirmDialog from '$lib/components/ConfirmDialog.svelte';
	import ChannelDialog from '$lib/components/ChannelDialog.svelte';
	import ChannelCard from '$lib/components/ChannelCard.svelte';
	import { Button, Pagination, PaginationItem } from 'flowbite-svelte';
	import { page } from '$app/stores';
	import { ChevronLeftOutline, ChevronRightOutline } from 'flowbite-svelte-icons';
	import { GradientButton } from 'flowbite-svelte';
	import { CirclePlusSolid } from 'flowbite-svelte-icons';
	import type { Channel } from '$lib/types';
	import { per_page } from '$lib/global';
	import { base_endpoint } from '$lib/global';
	export let showConfirmDialog = false;
	export let showChannelDialog = false;

	export let data: PageData;
	let channel: Channel;
	let onDialogButtonClicked: any;

	let channels: Channel[] = data.channels as Channel[];

	async function deleteChannel(channelToDelete: any) {
		console.log('deleteChannel');
		console.log(channelToDelete);
		const request = await fetch(
			`${base_endpoint}/api/1.0/channels/?channel_id=${channelToDelete.id}`,
			{
				method: 'DELETE',
				headers: {
					Accept: 'application/json'
				}
			}
		);
		const response = await request.json();
		if (response.status) {
			channels = channels.filter((item) => item.id != channelToDelete.id);
		}
	}

	async function updateChannel(channelToUpdate: Channel) {
		console.log('updateChannel');
		console.log(channelToUpdate);
		const request = await fetch(`${base_endpoint}/api/1.0/channels/`, {
			method: 'PUT',
			headers: {
				Accept: 'application/json',
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(channelToUpdate)
		});
		const response = await request.json();
		if (response.status) {
			channel = response.data;
		}
	}

	async function newChannel(newChannel: Channel) {
		console.log('newChannel');
		console.log(newChannel);
		const request = await fetch(`${base_endpoint}/api/1.0/channels/`, {
			method: 'POST',
			headers: {
				Accept: 'application/json',
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(newChannel)
		});
		const response = await request.json();
		console.log(response);
		if (response.status) {
			channel = response.data as Channel;
			console.log(channel);
			channels = [...channels, channel];
		}
	}

	function onUpdateChannelButtonClicked(channelToUpdate: Channel) {
		console.log(channelToUpdate);
		channel = channelToUpdate;
		onDialogButtonClicked = () => updateChannel(channel);
		showChannelDialog = true;
	}
	function onDeleteChannelButtonClicked(channel: Channel) {
		console.log('onDeleteChannelButtonClicked');
		deleteChannel(channel);
	}
	function onNewChannelButtonClicked() {
		console.log('onNewChannelButtonClicked');
		channel = {
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
		onDialogButtonClicked = () => newChannel(channel);
		showChannelDialog = true;
	}

	function getCurrentPage() {
		let currentPage = 1;
		const currentPageString = $page.url.searchParams.get('page');
		if (currentPageString != null) {
			currentPage = parseInt(currentPageString);
		}
		console.log(`currentPage: ${currentPage}`);
		return currentPage;
	}

	function getPaginatedChannels() {
		const total = channels.length;
		let start = (currentPage - 1) * per_page;
		if (start > total - 1) {
			start = total - 1;
		}
		let end = start + per_page;
		if (end > total) {
			end = total;
		}
		console.log(`start: ${start}, end: ${end}, total: ${total}`);
		return channels.slice(start, end);
	}

	function getPages() {
		const path = $page.url.pathname;
		const href = $page.url.href;
		console.log(`url: ${path}`);
		const pages = [];
		const max_page = channels.length / per_page;
		for (let i = currentPage - 2; i <= currentPage + 2; i++) {
			if (i > 0 && i <= max_page) {
				console.log(`page ${i}`);
                console.log($page.url);
				console.log(`${href}?page=${i}`);
				pages.push({ name: `${i}`, href: `${href}?page=${i}`, page: i });
			}
		}
		console.log(pages);
		return pages;
	}
	$: currentPage = getCurrentPage();
	$: paginatedChannels = getPaginatedChannels();
	$: pages = getPages();
</script>

<div id="channels" class="grid justify-items-center">
	<GradientButton on:click={onNewChannelButtonClicked} class="mb-4">
		<CirclePlusSolid />
	</GradientButton>
	{#each paginatedChannels as channel}
		<ChannelCard {channel} {onUpdateChannelButtonClicked} {onDeleteChannelButtonClicked} />
	{/each}
</div>
<div class="grid justify-items-center">
    {#each pages as page}
        <Button on:click={
            () => {
                currentPage = page.page;
                console.log(currentPage);
                paginatedChannels = getPaginatedChannels();
            }
        }>{page.name}</Button>
    {/each}
</div>

<div class="grid justify-items-center">
	<Pagination {pages} large>
		<svelte:fragment slot="prev">
			<span class="sr-only">Previous</span>
			<ChevronLeftOutline class="w-6 h-6" />
		</svelte:fragment>
		<svelte:fragment slot="next">
			<span class="sr-only">Next</span>
			<ChevronRightOutline class="w-6 h-6" />
		</svelte:fragment>
	</Pagination>
</div>

<ChannelDialog bind:open={showChannelDialog} {channel} onOkButtonClicked={onDialogButtonClicked} />

<ConfirmDialog
	bind:open={showConfirmDialog}
	title="Warning"
	message="Are you sure?"
	onOkButtonClicked={deleteChannel}
	on:close={() => deleteChannel(channel.id)}
></ConfirmDialog>
