<script lang="ts">
    import { goto } from '$app/navigation';
    import { page } from '$app/stores';
    import { loading } from '$lib/stores/loading.store';
    import { notification } from '$lib/stores/notification.store';
    import { isAuthenticated, loggedInUser } from '$lib/stores/user.store';
    import { BASE_API_URI, happyEmoji } from '$lib/utils/constant';
    import type { CustomError, User } from '$lib/utils/types';
    import { flip } from 'svelte/animate';
    import { scale } from 'svelte/transition';
    import { base_endpoint } from '$lib/global';
    let username = '',
        password = '',
        errors: Array<CustomError> = [];
    const handleLogin = async () => {
        console.log("handleLogin");
        loading.setLoading(true, 'Please wait while we log you in...');
        try {
            const response = await fetch(`${base_endpoint}/api/1.0/login/`, {
                method: 'POST',
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    username: username,
                    password: password,
                })

            });
            const content = await response.json();
            loading.setLoading(false);
            console.log(content);
            if (content.status){
                const response: User = content.data as User;
                $loggedInUser = {
                    id: response.id,
                    name: response.name,
                    role: response.role,
                    active: response.active,
                };
                $isAuthenticated = true;
                $notification = {
                    message: `Login successfull ${happyEmoji}...`,
                    colorName: `green`
                };
                console.log($page.url);
                console.log($page.url.hash);
                let nextPage = $page.url.search.split('=')[1];
                if ($page.url.hash) {
                    nextPage = `${nextPage}${$page.url.hash}`;
                }
                await goto(nextPage || '/', { noScroll: true });
            }else{
                $notification = {
                    message: `Erro successfull ${happyEmoji}...`,
                    colorName: `red`
                };
            }
        } catch(error) {
            console.log(error);
            loading.setLoading(false);
            $notification = {
                message: `Erro successfull ${happyEmoji}...`,
                colorName: `red`
            };
            errors = error;
        }
    };
</script>

<svelte:head>
    <title>Auth - Login | Actix Web & SvelteKit</title>
</svelte:head>

<div class="flex items-center justify-center h-[60vh]">
    <form
        class="w-11/12 md:w-2/3 lg:w-1/3 rounded-xl flex flex-col items-center align-middle bg-slate-800 py-4"
        on:submit|preventDefault={handleLogin}
    >
        <h1 class="text-center text-2xl font-bold text-sky-400 mb-6">Login</h1>

        {#if errors}
            {#each errors as error (error.id)}
                <p
                    class="text-center text-rose-600"
                    transition:scale|local={{ start: 0.7 }}
                    animate:flip={{ duration: 200 }}
                >
                    {error.error}
                </p>
            {/each}
        {/if}

        <div class="w-3/4 mb-2">
            <input
                type="text"
                name="username"
                id="username"
                bind:value={username}
                class="w-full text-sky-500 placeholder:text-slate-600 border-none focus:ring-0 bg-main-color focus:outline-none py-2 px-3 rounded"
                placeholder="User name"
                required
            />
        </div>

        <div class="w-3/4 mb-6">
            <input
                type="password"
                name="password"
                id="password"
                bind:value={password}
                class="w-full text-sky-500 placeholder:text-slate-600 border-none focus:ring-0 bg-main-color focus:outline-none py-2 px-3 rounded"
                placeholder="Password"
                required
            />
        </div>

        <div class="w-3/4 mt-4">
            <button
                type="submit"
                class="py-2 bg-sky-800 w-full rounded text-blue-50 font-bold hover:bg-sky-700"
            >
                Login
            </button>
        </div>
    </form>
</div>
