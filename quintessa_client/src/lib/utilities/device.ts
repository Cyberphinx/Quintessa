// src/lib/utils/device.ts
import { derived, writable } from 'svelte/store';
import { browser } from '$app/environment';

// Default to 0 width - this won't be updated server side because svelte:window won't work server side
export const width = writable<number>(0);
// Default to 0 height - this won't be updated server side because svelte:window won't work server side
export const height = writable<number>(0);

// This will be set server side, from the __layout.svelte component
export const mobile = writable<boolean>(true);

// This store computes what device to render for
export const device = derived([width, mobile], ([width, mobile]) => {
    if (width > 1200 || (!browser && !mobile)) {
        return 'desktop';
    } else if (width > 1000) {
        return 'laptop';
    } else if (width > 460) {
        return 'tablet';
    } else {
        return 'mobile';
    }
});
