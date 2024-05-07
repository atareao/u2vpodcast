import type { PageLoad } from './$types';
import { base_endpoint } from '$lib/global';
import type { Response, Channel } from '$lib/types';
import { redirect } from '@sveltejs/kit';
import { base } from '$app/paths';
import { isAuthenticated, loggedInUser} from '$lib/stores/user.store';

export const load: PageLoad = async ({ fetch, route }) => {
    const ans = await fetch(`${base_endpoint}/api/1.0/channels/`);
    const response: Response = await ans.json();
    if (response.user == null) {
        const redirectUrl = `${base}/login?next=${base}${route.id}`; 
        console.log(`redirectUrl: ${redirectUrl}`);
        redirect(302, redirectUrl);
    }else{
        isAuthenticated.set(true);
        loggedInUser.set(response.user);
    }
    return {
        channels: response.data as Channel[],
    };
};
