import { writable } from 'svelte/store';
import type { SessionUser } from '$lib/user';

export type AuthStore = {
    initializing: boolean;
    sessionUser: SessionUser | null | undefined;
};

const authStore = writable<AuthStore>({
    initializing: true,
    sessionUser: undefined
});

export default authStore;
