<script lang="ts">
	import { Modal, Button, Toggle, Label, Input } from 'flowbite-svelte';
	export let isNewFormOpen: boolean;
	export let active: boolean = true;
	export let url: string = '';
	export let max: number = 5;
    export let first: Date = new Date();
    console.log(first);

	function onChangeFirst(e: any) {
		console.log(e);
	}

    function onButtonClicked(){
        isNewFormOpen = false;
    }
    function onSubmit(){
        console.log("Submit");
    }
</script>

<Modal 
    bind:open={isNewFormOpen}
    outsideclose
    on:close={() => {
        isNewFormOpen = false;
        console.log(isNewFormOpen);
    }}
    size="xs" class="w-full">
	<form class="flex flex-col space-y-6">
		<h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">New Channel</h3>
		<!--
        active: boolean
        url: string
        first: date
        max: number
        -->
		<Toggle bind:checked={active}>Active</Toggle>
		<Label class="space-y-2">
			<span>Url</span>
			<Input type="url" name="url" placeholder="url" bind:value={url} required />
		</Label>
		<Label class="space-y-2">
			<span>Max number of episodes</span>
			<Input type="number" name="max" placeholder="max" min="1" bind:value={max} required />
		</Label>
		<Label class="space-y-2">
			<span>First episode date</span>
			<Input
                type="date"
                name="first"
                placeholder="first"
                bind:value={first}
				on:change={onChangeFirst}
				on:input={(e) => (first = e.target.value)}
                required
                />
		</Label>
		<Button
            on:click={onButtonClicked}
            >Create channel</Button>
	</form>
</Modal>
