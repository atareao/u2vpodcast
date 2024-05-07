import type { PageLoad } from './$types';
import { base_endpoint } from '$lib/global';
import type { Response, Episode } from '$lib/types';
import { redirect } from '@sveltejs/kit';
import { base } from '$app/paths';
import { isAuthenticated, loggedInUser} from '$lib/stores/user.store';

export const load: PageLoad = async ({ fetch, route, params }) => {
    console.log(params);
    console.log(params.id);
    console.log(`base: ${base}`);
    console.log(route.id);
    console.log("=== episodes ===");
    const url = `${base_endpoint}/api/1.0/channels/${params.id}/episodes/`;
    console.log(url)
    const ans = await fetch(url);
    const response: Response = await ans.json();
    console.log(response);
    if (response.user == null) {
        const redirectUrl = `${base}/login?next=${base}${route.id}`; 
        console.log(`redirectUrl: ${redirectUrl}`);
        redirect(302, redirectUrl);
    }else{
        isAuthenticated.set(true);
        loggedInUser.set(response.user);
    }
    return {
        episodes: response.data as Episode[]
    };
};
