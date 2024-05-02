import { writable } from 'svelte/store';

export const isAuthenticated = writable(false);
export const loggedInUser = writable();
export const auth = writable({
    isAuth: false,
    user: null,
})
