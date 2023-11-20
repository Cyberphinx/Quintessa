<script lang="ts">
    import { theme } from "$lib/stores/commonStore";
    import { width, mobile, height } from "$lib/utilities/device";
    import { onMount } from "svelte";

    export let data: any;
    $mobile = data.overrideMobile;
    $: theme.set(data.theme);

    onMount(() => {});
</script>

<svelte:window bind:innerWidth={$width} bind:innerHeight={$height} />

<svelte:head>
    <meta name="color-scheme" content="light dark" />
    <link
        rel="prefetch"
        href="/fonts/Noto_Sans/NotoSans-Regular.ttf"
        as="font"
        type="font/ttf"
        crossorigin="anonymous"
    />
    <link rel="stylesheet" href={`/themes/${$theme}.css`} as="style" />
</svelte:head>
<slot />

<style>
    @font-face {
        font-family: "Noto Sans";
        font-style: normal;
        font-weight: 400;
        src: url("/fonts/Noto_Sans/NotoSans-Regular.ttf") format("truetype");
    }
    :global(body) {
        background: var(--base);
        color: var(--text);
        font-family: "Noto Sans", sans-serif;
    }
    :global(ul) {
        list-style-type: none;
    }
    :global(svg) {
        cursor: pointer;
        color: var(--text);
    }
    :global(button) {
        cursor: pointer;
        background: none;
        border: none;
        font-size: 1rem;
    }
</style>
