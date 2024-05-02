<script lang="ts">
	import {
		Button,
		DarkMode,
		Navbar,
		NavBrand,
	} from 'flowbite-svelte';
	import '../app.css';
    import { goto } from '$app/navigation';
	import logo from '$lib/assets/favicon/favicon-48x48.png';
	let btnClass = 'text-primary-500 dark:text-primary-600 border dark:border-gray-800';

    async function logout(){
        const ans = await fetch("/api/1.0/logout/");
        console.log(ans);
    }
    function handleLogout() {
        logout();
        goto("/app/login/");
    }
</script>

<Navbar class="mb-8">
	<NavBrand href="/">
		<img src={logo} class="mr-3 h-6 sm:h-9" alt="U2VPodcast" />
		<span class="self-center whitespace-nowrap text-xl font-semibold dark:text-white">
			U2VPodcast
		</span>
	</NavBrand>
	<Button on:click={handleLogout}>Logout</Button>
	<DarkMode {btnClass} />
</Navbar>
<slot />
