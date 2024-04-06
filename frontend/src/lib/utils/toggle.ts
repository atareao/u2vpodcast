import type { Writable } from 'svelte/store';

const toggle = (store: Writable<boolean>) => {
  store.update((s) => !s);
};

export default toggle;
