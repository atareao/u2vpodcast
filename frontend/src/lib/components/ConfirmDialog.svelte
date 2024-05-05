<script lang="ts">
	import { Button, Modal } from 'flowbite-svelte';
	import { CheckOutline, CloseOutline } from 'flowbite-svelte-icons';
	export let open: boolean;
	export let onOkButtonClicked: any;
	export let title: string;
	export let message: string;

	function handleOkButtonClicked() {
		open = false;
		onOkButtonClicked();
	}
	function handleCancelButtonClicked() {
		open = false;
	}
</script>

<Modal bind:open size="xs">
	<div>
		{#if title && title != ''}
			<h3><strong>{title}</strong></h3>
		{/if}
		<span>{message}</span>
	</div>
	<div class="block space-y-4 md:space-y-0 md:space-x-4">
		<Button on:click={handleOkButtonClicked}><CheckOutline /></Button>
		<Button on:click={handleCancelButtonClicked}><CloseOutline /></Button>
	</div>
</Modal>
