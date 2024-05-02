<script lang="ts">
	import { onMount } from 'svelte';
	import {
		Button,
		Dropdown,
		DropdownItem,
		DarkMode,
		Navbar,
		NavBrand,
		NavLi,
		NavUl,
		NavHamburger
	} from 'flowbite-svelte';
	import { ChevronDownOutline } from 'flowbite-svelte-icons';
	import '../app.css';
	import logo from '$lib/assets/favicon/favicon-48x48.png';
    import { isAuthenticated, loggedInUser } from '$lib/stores/user.store';
    import { get } from 'svelte/store';
	import LoginForm from '$lib/components/LoginForm.svelte';

	let btnClass = 'text-primary-500 dark:text-primary-600 border dark:border-gray-800';
	let loginFormShow = false;
    console.log("===========================");
    console.log($isAuthenticated);
    console.log(isAuthenticated);
    console.log(get(isAuthenticated));
    console.log("===========================");
</script>

<Navbar class="mb-8" let:hidden let:toggle>
	<NavBrand href="/">
		<img src={logo} class="mr-3 h-6 sm:h-9" alt="U2VPodcast" />
		<span class="self-center whitespace-nowrap text-xl font-semibold dark:text-white">
			U2VPodcast
		</span>
	</NavBrand>
	<NavHamburger on:click={toggle} />
	<NavUl {hidden}>
		<NavLi href="/" active={true}>Home</NavLi>
        {#if $loggedInUser }
		<NavLi class="cursor-pointer">
			Configuration<ChevronDownOutline
				class="w-3 h-3 ms-2 text-primary-800 dark:text-white inline"
			/>
			<Dropdown class="w-44 z-20">
				<DropdownItem href="/app/configure/channels">Channels</DropdownItem>
				<DropdownItem href="/app/configure/users">Users</DropdownItem>
				<DropdownItem href="/app/configure/general">General</DropdownItem>
			</Dropdown>
		</NavLi>
        {/if}
		<Button on:click={() => (loginFormShow = true)}>Login</Button>
		<DarkMode {btnClass} />
	</NavUl>
</Navbar>
<slot />
<!--
<LoginForm bind:open={loginFormShow} />
-->
