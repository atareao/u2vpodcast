<script lang="ts">
	import type { Channel } from '$lib/utils/types';
	import { Modal, Button, Toggle, Label, Input } from 'flowbite-svelte';

	export let open: boolean = false;
	export let channel: Channel;
	export let onOkButtonClicked: any;

	function getDate() {
        let response: string;
		if (!channel) {
            console.log("No channel");
            response = new Date().toISOString().split("T")[0];
		}else if(typeof(channel.first) == "string"){
            response = channel.first.split('T')[0];
        }else{
		    response = channel.first.toISOString().split('T')[0];
        }
        console.log(`response: ${response}`);
        return response;
	}
    console.log(channel);

	function handleOkButtonClicked() {
		onOkButtonClicked();
		open = false;
	}
	function handleCancelButtonClicked() {
		open = false;
	}

	$: firstDate = getDate();
</script>

<Modal bind:open size="xs" class="w-full">
	<form class="flex flex-col space-y-6" action="">
		{#if channel.id == 0}
			<h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">New Channel</h3>
		{:else}
			<h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">Edit channel</h3>
		{/if}
		<Toggle bind:checked={channel.active}>Active</Toggle>
		<Label class="space-y-2">
			<span>Url</span>
            {#if channel.id == 0}
			<Input
				type="url"
				name="url"
				placeholder="url"
				bind:value={channel.url}
				required
			/>
            {:else}
			<Input
				type="url"
				name="url"
				placeholder="url"
				bind:value={channel.url}
				readonly
				required
			/>
            {/if}
		</Label>
		<Label class="space-y-2">
			<span>Max number of episodes</span>
			<Input
				type="number"
				name="max"
				placeholder="max"
				min="-1"
				bind:value={channel.max}
				on:input={(e) => {
					if (e.target != null && e.target.value != null) {
						channel.max = parseInt(e.target.value);
					}
				}}
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
				on:input={(e) => {
					if (e.target != null && e.target.value != null) {
						channel.first = new Date(e.target.value);
					}
				}}
				required
			/>
		</Label>
        <div class="flex flex-row md:space-y-0 md:space-x-4">
            {#if channel.id == 0}
                <Button on:click={handleOkButtonClicked}>Create channel</Button>
            {:else}
                <Button on:click={handleOkButtonClicked}>Update channel</Button>
            {/if}
            <Button on:click={handleCancelButtonClicked}>Cancel</Button>
        </div>
	</form>
</Modal>
