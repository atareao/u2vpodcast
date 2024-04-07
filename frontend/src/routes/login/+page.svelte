<script lang="ts">
    import { Button, Modal, Label, Input } from 'flowbite-svelte';
	import type { PageData, ActionData } from './$types';
	
	export let data: PageData;
	
	export let form: ActionData;
    const loginEndpoint = "/login/";
</script>

{#if form?.success}
	<!-- this message is ephemeral; it exists because the page was rendered in
		   response to a form submission. it will vanish if the user reloads -->
	<p>Successfully logged in! Welcome back, {data.user.name}</p>
{/if}
<form class="flex flex-col space-y-6" action={loginEndpoint} method="POST">
	{#if form?.missing}<p class="error">The email field is required</p>{/if}
	{#if form?.incorrect}<p class="error">Invalid credentials!</p>{/if}
    <h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">Sign in</h3>
    <Label class="space-y-2">
      <span>User</span>
      <Input type="text" name="username" placeholder="username" value={form?.username ?? ""} required />
    </Label>
    <Label class="space-y-2">
      <span>Password</span>
      <Input type="password" name="password" placeholder="•••••" required />
    </Label>
    <Button type="submit" class="">Login</Button>
</form>
