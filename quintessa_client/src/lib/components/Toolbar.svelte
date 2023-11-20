<script lang="ts">
    import { page } from "$app/stores";
    import { theme, toaster } from "$lib/stores/commonStore";
    import { onMount } from "svelte";

    export let form: any;

    const time = new Intl.DateTimeFormat("en", { timeZoneName: "long" });
    const date = new Date();

    const year = new Date().getFullYear();
    let now = new Date().toLocaleTimeString();

    // using onMount keeps the setup and cleanup code together.
    // It also ensures that the interval is not started until the component is actually in the DOM.
    onMount(() => {
        let interval = setInterval(() => {
            now = new Date().toLocaleTimeString();
        }, 1000);
        return () => clearInterval(interval);
    });

    async function setTheme(themeValue: string) {
        theme.set(themeValue);
        await fetch(`${$page.url.origin}/api/set_theme`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(themeValue),
        });
    }

    function handleClick() {
        if ($theme === "dark") {
            setTheme("light");
            theme.set("light");
        } else {
            setTheme("dark");
            theme.set("dark");
        }
    }
</script>

<div class="toolbar">
    <p class="title">
        © {year}
        <span class="datetime"
            ><span class="date">{time.format(date)},</span> {now}
        </span>
        <span class="theme"
            ><button aria-label="Light theme" on:click={() => handleClick()}
                >{#if $theme === "light"}<svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="24"
                        height="24"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        class="lucide lucide-moon"
                        ><path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" /></svg
                    >{:else}<svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="24"
                        height="24"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        class="lucide lucide-sun"
                        ><circle cx="12" cy="12" r="4" /><path
                            d="M12 2v2"
                        /><path d="M12 20v2" /><path
                            d="m4.93 4.93 1.41 1.41"
                        /><path d="m17.66 17.66 1.41 1.41" /><path
                            d="M2 12h2"
                        /><path d="M20 12h2" /><path
                            d="m6.34 17.66-1.41 1.41"
                        /><path d="m19.07 4.93-1.41 1.41" /></svg
                    >{/if}</button
            ></span
        >
        {#if form && form.success}
            <span class="success">{$toaster}</span>
        {/if}
    </p>
</div>

<style>
    .toolbar {
        position: sticky;
        top: 0;
        margin: 0;
        padding: 1rem;
        border-bottom: 1px solid var(--highlight-med);
        border-radius: 0.5rem 0.5rem 0 0;
        background-color: var(--overlay);
    }
    .title {
        margin: 0;
    }
    .theme {
        position: absolute;
        right: 1rem;
        top: 0.85rem;
    }

    .success {
        position: absolute;
        top: 0.85rem;
        right: 4rem;
        color: var(--base);
        background-color: var(--foam);
        border-radius: 0.5rem;
        font-size: 0.85rem;
        padding: 0.3rem 0.5rem;
        animation: hideAfterDelay 5s forwards;
    }
    @keyframes hideAfterDelay {
        0% {
            opacity: 1;
        }
        100% {
            opacity: 0;
            visibility: hidden;
        }
    }

    .datetime {
        color: #908caa;
        font-size: 0.95rem;
        padding-left: 0.95rem;
    }

    @media screen and (max-width: 1000px) {
        .date {
            display: none;
        }
    }
</style>
