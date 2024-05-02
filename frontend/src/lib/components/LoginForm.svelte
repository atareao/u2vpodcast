<script lang="ts">
	import { Button, Modal, Label, Input } from 'flowbite-svelte';
    import { enhance, applyAction } from '$app/forms';
    import { invalidateAll } from "$app/navigation";
	import type { PageData, ActionData } from './$types';
	export let open: boolean;
	export let data: PageData;
	export let form: ActionData;
	const loginEndpoint = '/api/1.0/login/';
</script>

<Modal bind:open autoclose outsideclose size="xs" class="w-full">
	<form
		class="flex flex-col space-y-6"
		action={loginEndpoint}
		method="POST"
		use:enhance={() => {
			return async ({ result }) => {
				await invalidateAll();
				await applyAction(result);
			};
		}}>
		{#if form?.missing}<p class="error">The email field is required</p>{/if}
		<h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">Sign in</h3>
		<Label class="space-y-2">
			<span>User</span>
			<Input
				type="text"
				name="username"
				placeholder="username"
				value={form?.username ?? ''}
				required
			/>
		</Label>
		<Label class="space-y-2">
			<span>Password</span>
			<Input type="password" name="password" placeholder="•••••" required />
		</Label>
		<Button type="submit" class="">Login</Button>
	</form>
</Modal>
