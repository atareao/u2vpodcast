import type { PageLoad } from './$types';
import { base_endpoint } from '$lib/global';
import type { Response, Channel } from '$lib/utils/types';
import { redirect } from '@sveltejs/kit';
import { base } from '$app/paths';
import { isAuthenticated, loggedInUser} from '$lib/stores/user.store';

export const load: PageLoad = async ({ fetch, route, params }) => {
    console.log(`params: ${params}`);
    console.log(params);
    console.log(`base: ${base}`);
    console.log(route.id);
    console.log("=== channels ===");
    const ans = await fetch(`${base_endpoint}/api/1.0/channels/`);
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
        channels: response.data as Channel[]
    };
};

