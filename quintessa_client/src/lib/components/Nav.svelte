<script>
    import { enhance } from "$app/forms";
    import {
        isAuthenticated,
        loading,
        toaster,
        showCv,
    } from "$lib/stores/commonStore";
    import { clickOutside } from "$lib/utilities/clickOutside";
    import { device, width } from "$lib/utilities/device";
    import Menu from "./Menu.svelte";

    let menu = false;
</script>

<div class="container">
    <button
        class="navigation"
        style={$showCv
            ? "background-color: var(--foam); border: 1px solid var(--pine); color: var(--base);"
            : ""}
        on:click={() => showCv.set(true)}>CV</button
    >
    <button
        class="navigation"
        style={$showCv
            ? ""
            : "background-color: var(--foam); border: 1px solid var(--pine); color: var(--base);"}
        on:click={() => showCv.set(false)}>Portfolio</button
    >

    {#if $device === "mobile" || $width < 661}
        <div use:clickOutside on:clickoutside={() => (menu = false)}>
            <button class="menu" on:click={() => (menu = !menu)}
                ><svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="20"
                    height="20"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    class="lucide lucide-menu"
                    ><line x1="4" x2="20" y1="12" y2="12" /><line
                        x1="4"
                        x2="20"
                        y1="6"
                        y2="6"
                    /><line x1="4" x2="20" y1="18" y2="18" /></svg
                ></button
            >
            {#if menu}<Menu />{/if}
        </div>{/if}
    {#if $isAuthenticated}
        <form
            class="logout-form"
            action="?/logout"
            method="POST"
            use:enhance={() => {
                loading.set(true);
                return async ({ update }) => {
                    await update();
                    toaster.set("Successfully logged out!");
                    loading.set(false);
                };
            }}
        >
            <button class="logout" type="submit" id="logout">Logout</button>
        </form>
    {/if}
</div>

<style>
    .container {
        margin-bottom: 1rem;
        display: flex;
        gap: 1rem;
        justify-content: right;
    }

    .navigation {
        color: var(--text);
        background-color: var(--highlight-low);
        padding: 0.5rem 1rem;
        border-radius: 0.5rem;
        border: 1px solid var(--highlight-high);
        box-shadow: 3px 3px 3px rgba(0, 0, 0, 0.5);
    }

    .menu {
        color: var(--text);
        background-color: var(--highlight-low);
        padding: 0.5rem 0.75rem 0.25rem 0.75rem;
        border-radius: 0.5rem;
        border: 1px solid var(--highlight-high);
        box-shadow: 3px 3px 3px rgba(0, 0, 0, 0.5);
    }

    .logout {
        color: var(--base);
        font-weight: bold;
        padding: 0.5rem 1rem;
        border-radius: 0.5rem;
        background-color: var(--love);
        border: 1px solid transparent;
        box-shadow: 3px 3px 3px rgba(0, 0, 0, 0.5);
    }
    .logout:hover {
        background-color: var(--rose);
    }

    @media screen and (max-width: 660px) {
        .container {
            justify-content: space-between;
        }
        .navigation {
            display: none;
        }
    }
</style>
