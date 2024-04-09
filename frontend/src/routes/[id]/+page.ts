import { base_endpoint } from '$lib/global';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch, params }) => {
    console.log(params.id);
    const endpoint = `${base_endpoint}/api/1.0/channels/${params.id}/episodes/`;
    const request = await fetch(endpoint);
    const episodes = await request.json();
    console.log(episodes);
    return { episodes: episodes };
};
