<script lang="ts">
	import { onMount } from 'svelte';
    import { type Channel, } from '$lib/channel';
    import ChannelCardEditor from '$lib/components/ChannelCardEditor.svelte';
    import { base_endpoint } from '$lib/global';
	const endpoint = `${base_endpoint}/api/1.0/channels/`;
    let isFormOpen: boolean;
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
</script>

<div class="grid justify-items-center">
{#each channels as channel}
    <ChannelCardEditor {channel} />
{/each}
</div>




