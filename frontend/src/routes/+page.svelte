<script lang="ts">
	import type { PageData } from './$types';
	import ConfirmDialog from '$lib/components/ConfirmDialog.svelte';
	import ChannelDialog from '$lib/components/ChannelDialog.svelte';
	import ChannelCard from '$lib/components/ChannelCard.svelte';
    import { page } from '$app/stores';
    import { Pagination } from 'flowbite-svelte';
    import { ChevronLeftOutline, ChevronRightOutline } from 'flowbite-svelte-icons';
	import { GradientButton } from 'flowbite-svelte';
	import { CirclePlusSolid } from 'flowbite-svelte-icons';
	import type { Channel, Response, PaginationData } from '$lib/utils/types';
	import { base_endpoint } from '$lib/global';
	export let showConfirmDialog = false;
	export let showChannelDialog = false;

	export let data: PageData;
	let channel: Channel;
	let onDialogButtonClicked: any;

    let response: Response = data.response;
	let channels: Channel[] = response.data as Channel[];
    let paginationData: PaginationData = response.pagination as PaginationData;

	async function deleteChannel(channelToDelete: any) {
		console.log('deleteChannel');
		console.log(channelToDelete);
		const request = await fetch(
            `${base_endpoint}/api/1.0/channels/?channel_id=${channelToDelete.id}`,
            {
			method: 'DELETE',
			headers: {
				Accept: 'application/json',
			},
        });
		const response = await request.json();
        if(response.status){
            channels = channels.filter((item) => item.id != channelToDelete.id)
        }
	}

	async function updateChannel(channelToUpdate: Channel) {
		console.log('updateChannel');
		console.log(channelToUpdate);
		const request = await fetch(
            `${base_endpoint}/api/1.0/channels/`,
		    {
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
		const request = await fetch(
            `${base_endpoint}/api/1.0/channels/`,
            {
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
        console.log("onDeleteChannelButtonClicked");
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
    $: activeUrl = $page.url.searchParams.get("page");
    let current_page = 1;
    if ($page.url.searchParams.get("page") != null) {
        current_page = parseInt($page.url.searchParams.get("page"));
    }
    let total_pages = Math.round(paginationData.total / paginationData.per_page) + 1;
    if(current_page > total_pages) {
        current_page = total_pages;
    }else if(current_page < 1){
        current_page = 1;
    }
    console.log(`current_page=${current_page}`);
    let pages = [];
    console.log(`total pages=${total_pages}`);
    if (total_pages > 1){
        let p_2 = current_page - 2;
        if(p_2 > 0) {
            pages.push({name: p_2, href: `/app/?page=${p_2}`})
        }
        let p_1 = current_page - 1;
        if(p_1 > 0) {
            pages.push({name: p_1, href: `/app/?page=${p_1}`})
        }
        let n_1 = current_page + 1;
        if(n_1 < total_pages) {
            pages.push({name: n_1, href: `/app/?page=${n_1}`})
        }
        let n_2 = current_page + 2;
        if(n_2 < total_pages) {
            pages.push({name: n_2, href: `/app/?page=${n_2}`})
        }
    }
    $: {
    pages.forEach((page) => {
      let splitUrl = page.href.split('?');
      let queryString = splitUrl.slice(1).join('?');
      const hrefParams = new URLSearchParams(queryString);
      let hrefValue = hrefParams.get('page');
      if (hrefValue === activeUrl) {
        page.active = true;
      } else {
        page.active = false;
      }
    });
    pages = pages;
  }

  const previous = () => {
    alert('Previous btn clicked. Make a call to your server to fetch data.');
  };
  const next = () => {
    alert('Next btn clicked. Make a call to your server to fetch data.');
  };
</script>

<div id="channels" class="grid justify-items-center">
	<GradientButton on:click={onNewChannelButtonClicked} class="mb-4">
		<CirclePlusSolid />
	</GradientButton>
	{#each channels as channel}
		<ChannelCard {channel} {onUpdateChannelButtonClicked} {onDeleteChannelButtonClicked} />
	{/each}
</div>
{#if total_pages > 1}
<div class="grid justify-items-center">
    <Pagination {pages} large on:previous={previous} on:next={next} icon>
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
{/if}

<ChannelDialog bind:open={showChannelDialog} {channel} onOkButtonClicked={onDialogButtonClicked} />

<ConfirmDialog
	bind:open={showConfirmDialog}
	title="Warning"
	message="Are you sure?"
	onOkButtonClicked={deleteChannel}
	on:close={() => deleteChannel(channel.id)}
>
</ConfirmDialog>
