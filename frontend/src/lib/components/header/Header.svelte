<script lang="ts">
    import { page } from '$app/stores';
    import { logout } from '$lib/utils/requests/logout.requests';
</script>

<header aria-label="Page Header" class="mb-6">
    <nav class="mx-auto max-w-screen-xl px-4 py-4 sm:px-6 lg:px-8">
        <div class="flex items-center justify-end gap-4">
            <div class="flex items-center gap-4">
                <a
                    href="https://github.com/Sirneij"
                    class="block shrink-0 rounded-full bg-white p-2.5 text-gray-600 shadow-sm hover:text-gray-700"
                >
                    <span class="sr-only">Programmer</span>
                </a>
            </div>

            <span aria-hidden="true" class="block h-6 w-px rounded-full bg-gray-200" />

            <a
                href="/"
                class="block shrink-0 {$page.url.pathname === `/`
                    ? 'text-sky-500'
                    : 'text-white'} hover:text-sky-400"
            >
                /
            </a>
            <button
                type="button"
                class="text-white hover:text-sky-400"
                on:click={() => logout($page.data.fetch)}
            >
                Logout
            </button>
        </div>
    </nav>
</header>
