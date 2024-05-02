import { writable } from 'svelte/store';

export const isAuthenticated = writable(false);

export const loggedInUser = writable({
    id: 0,
    name: "",
    role: "",
    active: false,
})
