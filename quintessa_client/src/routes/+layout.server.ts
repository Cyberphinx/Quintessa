import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ request, cookies, locals }) => {
    const themeData = cookies.get('theme');
    let theme: string;
    if (themeData) theme = themeData;
    else theme = 'dark';

    // console.log(`Token: ${locals.token}`);

    let isMobile = request.headers.get('sec-ch-ua-mobile') === '?1';
    return {
        overrideMobile: isMobile,
        token: locals.token,
        theme: theme
    };
}
