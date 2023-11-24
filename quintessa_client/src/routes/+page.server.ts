import type { Actions, PageServerLoad } from "./$types";
import { env } from "$env/dynamic/private";
import * as agent from '$lib/agent';
import type { LoginDataResponse, Media, Project } from "$lib/model";
import type { SignedMedia } from "$lib/model";

const base = `${env.API_URI}/api`;
const supabaseUrl = env.SUPABASE_URL;
const supabaseKey = env.SUPABASE_KEY;
// const geoApiKey = env.IPGEO_APIKEY;
// let refreshTokenTimeout: any = null;

export const load: PageServerLoad = async ({ getClientAddress, request, cookies, url }) => {
    let addr = getClientAddress();
    let userAgent = request.headers.get('user-agent');
    let platform = request.headers.get('sec-ch-ua-platform');

    // fetch geolocation from ip address
    // let geolocation = await agent.get(`https://api.ipgeolocation.io/ipgeo?apiKey=${geoApiKey}&ip=${addr}`);
    // console.log(geolocation);

    let resume: any;
    let projects: any;
    let signedProjects: Project[] = [];
    let cvMedia: SignedMedia[] = [];
    let projectsMedia: SignedMedia[] = [];
    let jwt = cookies.get('jwt');
    if (jwt) {
        try {
            // call refresh token functions
            // startRefreshTokenTimer(jwt);

            // fetch CV from database
            resume = await agent.get(`${base}/resume/1`, cookies.get('jwt')).then((value) => {
                return value.data;
            });


            // get pre-signed supabase images for CV
            cvMedia = await agent.post(`${supabaseUrl}/storage/v1/object/sign/portfolio`, {
                "expiresIn": 864000,
                "paths":
                    [
                        "quintessa/profile.jpg",
                        "quintessa/snippet_01.png",
                        "quintessa/snippet_02.png",
                        "quintessa/snippet_03.png",
                        "quintessa/snippet_04.png",
                        "quintessa/snippet_05.png",
                    ]
            }, supabaseKey);

            // projects search params
            const params = new URLSearchParams();
            const projectCategory = url.searchParams.get('category');
            if (projectCategory) {
                params.delete('category');
                params.set('category', projectCategory);
            }

            // fetch projects from database
            let mediaNames: string[] = [];
            projects = await agent.get(`${base}/projects?${params}`, cookies.get('jwt')).then((value) => {
                value.data.map((x: Project) => {
                    x.media.map((image: Media) => {
                        mediaNames.push(`${x.category}/${image.url}`);
                    })
                })
                signedProjects = value.data;
                return value.data;
            });

            // pre-sign supabase images for projects and then replace the urls inside projects with pre-signed urls
            signedProjects = await agent.post(`${supabaseUrl}/storage/v1/object/sign/portfolio`, {
                "expiresIn": 864000,
                "paths": [...mediaNames],
            }, supabaseKey).then((value: SignedMedia[]) => {
                value.map((media: SignedMedia) => {
                    projects.map((project: Project, projectIndex: number) => {
                        project.media.map((projectMedia: Media, mediaIndex: number) => {
                            if (media.path === `${project.category}/${projectMedia.url}`) {
                                signedProjects[projectIndex].media[mediaIndex].url = `${supabaseUrl}/storage/v1${media.signedURL}`;
                            };
                        });
                    });
                });
                return signedProjects;
            });
            // console.log(`projects media: ${ projectsMedia }`);
        } catch (error: any) {
            console.log(error);
            if (error.status == '401') {
                console.log('Expired token has been deleted!');
                cookies.delete('jwt');
            } else if (error.status == '404') {
                console.log('Token was not found in cookies!');
            }
        }
    }

    return {
        clientAddr: addr,
        userAgent: userAgent,
        platform: platform,
        resume,
        cvMedia,
        projects: signedProjects,
        projectsMedia,
        supabaseCreds: { url: supabaseUrl, key: supabaseKey },
    };
}

export const actions: Actions = {
    login: async (event) => {
        // log the user in
        const formData = await event.request.formData();
        const email = formData.get('email');
        const password = formData.get('password');

        const loginRequest = {
            email,
            password
        }
        const loginResponse: LoginDataResponse = await agent.post(`${base}/users/login`, loginRequest);

        const value = loginResponse.data.token;
        event.cookies.set('jwt', value, { path: '/' })

        // throw redirect(307, '/portfolio');
        return { success: true }
    },
    logout: async (event) => {
        // log the user out
        event.cookies.delete('jwt', { path: '/' });
        event.locals.token = null;

        return { success: true }
    }
}

// define refresh token functions
// refresh token
// async function refreshToken(event: any) {
//     stopRefreshTokenTimer();
//     try {
//         let jwt = event.cookies.get('jwt');
//         console.log(`${Date.now()}: Old token is: ${jwt}`);
//
//         const origin = event.request.headers.get('origin');
//         console.log(`request origin is: ${origin}`)
//
//         // const loginResponse = await agent.post(`${base}/user/refresh_token`, {}, jwt);
//         const path = `${base}/user/refresh_token`;
//         console.log(`request url is: ${path}`);
//         const response = await fetch(path, {
//             method: 'POST',
//             headers: {
//                 'Content-Type': 'application/json',
//                 // 'set-cookie': 'refresh-token=',
//             },
//             body: null,
//             credentials: 'include'
//         }).then((res) => {
//             console.log(`status code: ${res.status}`);
//             console.log(`response headers: ${res.headers}`);
//             console.log(`response value: ${res}`);
//             return res;
//         });
//
//         const refreshResponse = await response.json();
//
//         const value = refreshResponse.data.token;
//         console.log(`${Date.now()}: Refreshed token: ${value}`);
//         event.cookies.set('jwt', value, { path: '/' });
//         console.log(`${Date.now()}: Start refresh token again.`)
//         await startRefreshTokenTimer(event, refreshResponse.data.token);
//     } catch (error) {
//         console.log(`${Date.now()}: Failed to refresh token`);
//         console.log(error);
//     }
// }
// // start refresh token timer
// async function startRefreshTokenTimer(event: any, jwt: string) {
//     // console.log(`jwt: ${jwt}`);
//     console.log(`jwt split by period: ${atob(jwt.split('.')[1])}`)
//     const jwtToken = JSON.parse(atob(jwt.split('.')[1]));
//     const expires = new Date(jwtToken.exp * 1000);
//     const timeout = expires.getTime() - Date.now() - (30 * 1000);
//     console.log(`${Date.now()}: The timeout is: ${timeout}`);
//     refreshTokenTimeout = setTimeout(() => refreshToken(event), timeout);
//     console.log({ refreshTimeout: refreshTokenTimeout });
// }
// // stop refresh token timer
// function stopRefreshTokenTimer() {
//     console.log(`${Date.now()}: Clear timout for refresh token temporarily.`);
//     clearTimeout(refreshTokenTimeout);
// }
