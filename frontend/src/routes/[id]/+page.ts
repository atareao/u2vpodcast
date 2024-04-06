import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch, params }) => {
    console.log(params.id);
    const endpoint = `http://localhost:6996/api/1.0/channels/${params.id}/episodes/`;
    const request = await fetch(endpoint);
    const episodes = await request.json();
    console.log(episodes);
    return { episodes: episodes };
};
