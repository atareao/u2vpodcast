import type { PageLoad } from './$types';
import { base_endpoint } from '$lib/global';
import type { User } from '$lib/utils/types';
import { redirect } from '@sveltejs/kit';
import { base } from '$app/paths';

export const load: PageLoad = async ({ route, params }) => {
    console.log(`params: ${params}`);
    console.log(params);
    console.log(`base: ${base}`);
    console.log(route.id);
    console.log("==============");
    const response = await fetch(`${base_endpoint}/api/1.0/session/`);
    const content = await response.json();
    console.log(content);
    const user: User = content.data as User;
    const redirectUrl = `${base}/auth/login?next=${base}${route.id}`; 
    console.log(`redirectUrl: ${redirectUrl}`);
    if (user.id == 0) {
        redirect(302, redirectUrl);
    }
    return user;
};
