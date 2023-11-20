// import type { LoginDataResponse, LoginResponse } from "$lib/model";
// import { writable, type Writable } from "svelte/store";
// import * as agent from '$lib/agent';
// import { env } from "$env/dynamic/private";
//
// const base = `${env.API_URI}/api`;
//
// // refresh token
// type RefreshTokenStore = { refreshTokenTimeout: any; }
//
// function createRefreshTokenStore() {
//     const { subscribe, update }: Writable<RefreshTokenStore> = writable<RefreshTokenStore>({ refreshTokenTimeout: null });
//
//     async function refreshToken(event: any) {
//         stopRefreshTokenTimer();
//         try {
//             const loginResponse: LoginDataResponse = await agent.post(`${base}/user/refresh_token`, {});
//             const value = loginResponse.data.token;
//             event.cookies.set('jwt', value, { path: '/' });
//             startRefreshTokenTimer(loginResponse.data);
//         } catch (error) {
//             console.log(error);
//         }
//     }
//
//     function startRefreshTokenTimer(user: LoginResponse) {
//         const jwtToken = JSON.parse(atob(user.token.split('.')[2]));
//         const expires = new Date(jwtToken.exp * 1000);
//         const timeout = expires.getTime() - Date.now() - (30 * 1000);
//         update((store) => {
//             store.refreshTokenTimeout = setTimeout(refreshToken, timeout);
//             console.log({ refreshTimeout: store.refreshTokenTimeout });
//             return store;
//         })
//     }
//     function stopRefreshTokenTimer() {
//         update((store) => {
//             clearTimeout(store.refreshTokenTimeout);
//             return store;
//         })
//     }
//
//     return {
//         subscribe,
//         startRefreshTokenTimer
//     };
// }
// export const refreshTokenStore = createRefreshTokenStore();

// export const refreshTokenTimeout: Writable<any> = writable(null);

// export async function refreshToken(event: any) {
//     stopRefreshTokenTimer();
//     try {
//         const loginResponse: LoginDataResponse = await agent.post(`${base}/user/refresh_token`, {});
//         const value = loginResponse.data.token;
//         event.cookies.set('jwt', value, { path: '/' });
//         startRefreshTokenTimer(loginResponse.data);
//     } catch (error) {
//         console.log(error);
//     }
// }
//
// export function startRefreshTokenTimer(user: LoginResponse) {
//     const jwtToken = JSON.parse(atob(user.token.split('.')[2]));
//     const expires = new Date(jwtToken.exp * 1000);
//     const timeout = expires.getTime() - Date.now() - (30 * 1000);
//     refreshTokenTimeout.set(setTimeout(refreshToken, timeout));
//     console.log({ refreshTimeout: refreshTokenTimeout });
// }
//
// export function stopRefreshTokenTimer() {
//     clearTimeout($refreshTokenTimeout);
// }
