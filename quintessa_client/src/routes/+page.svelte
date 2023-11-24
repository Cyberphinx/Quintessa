<script lang="ts">
    import { enhance } from "$app/forms";
    import Portfolio from "$lib/components/Portfolio.svelte";
    import Cv from "$lib/components/Cv.svelte";
    import Top from "$lib/components/Top.svelte";
    import {
        isAuthenticated,
        loading,
        projects,
        toaster,
        you,
        portfolio,
        cv,
        cvMedia,
        showCv,
        resume,
    } from "$lib/stores/commonStore";
    import { device, height, width } from "$lib/utilities/device";
    import { onDestroy, onMount } from "svelte";
    import LoadingSpinner from "$lib/components/LoadingSpinner.svelte";
    import { supabaseCredentials } from "$lib/stores/supabaseStore";
    import Nav from "$lib/components/Nav.svelte";
    import Toolbar from "$lib/components/Toolbar.svelte";
    import { browser } from "$app/environment";
    import { goto } from "$app/navigation";

    export let data: any;
    export let form: any;
    $: data.token ? isAuthenticated.set(true) : isAuthenticated.set(false);
    $: data.resume ? resume.set(data.resume) : resume.set(undefined);
    $: data.cvMedia ? cvMedia.set(data.cvMedia) : cvMedia.set([]);
    $: data.projects ? projects.set(data.projects) : projects.set([]);
    $: data.supabaseCreds && supabaseCredentials.set(data.supabaseCreds);

    // check if scroll position is at top
    let atTop = true;
    function checkScrollPosition() {
        if (browser) {
            atTop = window.scrollY < 100;
        }
    }
    let scrollListener: any;
    onMount(() => {
        if (browser) {
            scrollListener = window.addEventListener(
                "scroll",
                checkScrollPosition,
            );
            checkScrollPosition();
        }
    });
    onDestroy(() => {
        if (browser) {
            window.removeEventListener("scroll", scrollListener);
        }
    });
</script>

{#if $loading}<div class="loading">
        <LoadingSpinner width={5} />
        <p class="loading-text">Loading portfolio...</p>
    </div>{/if}
{#if !atTop}<Top />{/if}
<div class="container">
    {#if data.token && $projects.length > 0}
        <Nav />
    {/if}

    {#if data.token && $resume !== undefined && $projects.length > 0}
        <div bind:this={$cv} style={$showCv ? "" : "display:none"}>
            <Toolbar {form} />
            <Cv />
        </div>
    {:else}
        <div bind:this={$cv}>
            <Toolbar {form} />
            <h1>Ling Leng</h1>
            <hr />
            <p class="quote">
                <i
                    >"A complex system that works is invariably found to have
                    evolved from a simple system that worked."</i
                >
            </p>
            <hr />
            <div class="sub-container">
                <p>UK ARB Qualified Architect / Full-Stack Developer</p>
                <p>
                    BArch <span style="color: #908caa"
                        >(Glasgow School of Art - Mackintosh School of
                        Architecture)</span
                    >
                </p>
                <p>
                    DipArch <span style="color: #908caa"
                        >(Architectural Association School of Architecture)</span
                    >
                </p>
                <p>
                    Previously: <span style="color: #908caa"
                        >Coop Himmelb(l)au, AHMM, Ocado Engineering
                    </span>
                </p>
            </div>
        </div>
    {/if}

    {#if data.token && $projects.length > 0}
        <div bind:this={$portfolio} style={$showCv ? "display:none" : ""}>
            <Toolbar {form} />
            <Portfolio />
        </div>
    {:else}
        <div bind:this={$portfolio}>
            <div class="toolbar">
                <p class="title">CV & Portfolio</p>
            </div>
            <div class="sub-container">
                <p>Please login to view further details:</p>
                <form
                    class="password-form"
                    action="?/login"
                    method="POST"
                    use:enhance={() => {
                        loading.set(true);
                        return async ({ update }) => {
                            await update();
                            toaster.set("Successfully logged in!");
                            loading.set(false);
                            goto("/");
                        };
                    }}
                >
                    <input
                        id="email"
                        class="input-box"
                        name="email"
                        placeholder="Email"
                        type="email"
                        required
                    />

                    <input
                        id="password"
                        class="input-box"
                        name="password"
                        placeholder="Password"
                        type="password"
                        required
                    />
                    <button class="submit" type="submit">Submit</button>
                </form>
            </div>
        </div>
    {/if}

    <div bind:this={$you}>
        <div class="toolbar">
            <p class="title">About you</p>
        </div>
        <div class="sub-container">
            {#if data.clientAddr}
                <p class="about">Your ip address: {data.clientAddr}</p>
            {/if}
            {#if $device}
                <p class="about">You are using a {$device} device</p>
            {/if}
            {#if $height && $width}
                <p class="about">
                    Your screen resolution {$width} x {$height}
                </p>
            {/if}
            {#if data.platform}
                <p class="about">
                    You are on {data.platform} operating system
                </p>
            {/if}
            {#if data.userAgent}
                <p class="about">Your user agent is {data.userAgent}</p>
            {/if}
        </div>
    </div>
</div>

<style>
    .loading {
        position: fixed;
        top: calc(50vh - 5rem);
        left: calc(50vw - 5rem);
        z-index: 100;
    }
    .loading-text {
        position: absolute;
        margin-top: 7rem;
        margin-left: -2.75rem;
        padding: 0.25rem 1rem;
        width: 150px;
        color: var(--base);
        background-color: var(--foam);
        border-radius: 0.5rem;
        font-weight: bold;
    }

    .container {
        position: relative;
        padding: 3rem 15vw;
        margin: auto;
    }

    .container > div {
        background-color: var(--surface);
        border: 1px solid var(--highlight-med);
        border-radius: 0.5rem;
        margin-bottom: 2rem;
    }

    .sub-container {
        margin: 3rem;
    }

    .toolbar {
        position: relative;
        margin: 0;
        padding: 1rem;
        border-bottom: 1px solid var(--highlight-med);
        border-radius: 0.5rem 0.5rem 0 0;
        background-color: var(--overlay);
    }
    .title {
        margin: 0;
    }

    .quote {
        margin: 3rem;
        color: var(--subtle);
    }

    hr {
        margin: 0 3rem;
        border: 0px solid var(--highlight-low);
        border-top: 1px solid var(--highlight-med);
    }

    h1 {
        margin: 3rem;
        font-size: 3rem;
    }

    .about {
        color: #908caa;
    }

    .password-form {
        display: flex;
        flex-wrap: wrap;
        gap: 1rem;
    }
    .input-box {
        border-radius: 8px;
        padding: 0.5rem 1rem;
        border: none;
        background-color: var(--highlight-high);
        font-size: 1rem;
        color: var(--text);
    }

    .submit {
        color: var(--text);
        font-size: 1rem;
        background-color: var(--overlay);
        padding: 0.5rem 1rem;
        border-radius: 8px;
        box-shadow: 3px 3px 3px rgba(0, 0, 0, 0.5);
    }
    .submit:hover {
        background-color: var(--highlight-low);
    }

    @media screen and (max-width: 1280px) {
        .container {
            margin: auto;
            padding: 1rem;
        }
        .sub-container {
            margin: 1rem;
        }
        hr {
            margin: 0 1rem;
        }
        h1 {
            margin: 1rem;
            font-size: 2.5rem;
        }
        .quote {
            margin: 3rem 1rem;
        }
    }
</style>
