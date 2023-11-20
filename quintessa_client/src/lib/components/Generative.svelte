<script lang="ts">
    import { currentAnimation } from "$lib/stores/commonStore";
    import Rain from "./Rain.svelte";
    import RandomWalk from "./RandomWalk.svelte";
    import Sine from "./Sine.svelte";
    import Vine from "./Vine.svelte";
    import Wave from "./Wave.svelte";

    let artworks = [
        { name: "Wave", description: "" },
        {
            name: "Rain",
            description:
                "Mouse horizontal location controls size of dots; Mouse vertical location controls opacity of dots",
        },
        { name: "Sine", description: "" },
        {
            name: "Vine",
            description:
                "Random shades of green crawling up the canvas; Soon enough it will fill up the canvas",
        },
        {
            name: "Random Walk",
            description:
                "Click in the middle of the canvas to start random walks on the canvas",
        },
    ];
</script>

<div class="container">
    <div class="projects-nav">
        <button
            on:click={() => {
                if ($currentAnimation > 0) $currentAnimation -= 1;
            }}>Previous <span class="extra-text">project</span></button
        >
        <p class="projects-index">
            {$currentAnimation + 1} of {artworks.length}
            <span class="extra-text">projects</span>
        </p>
        <button
            on:click={() => {
                if ($currentAnimation < artworks.length - 1)
                    $currentAnimation += 1;
            }}>Next <span class="extra-text">project</span></button
        >
    </div>
    <div>
        {#if $currentAnimation === 0}
            <Wave />
        {:else if $currentAnimation === 1}
            <Rain />
        {:else if $currentAnimation === 2}
            <Sine />
        {:else if $currentAnimation === 3}
            <Vine />
        {:else if $currentAnimation === 4}
            <RandomWalk />
        {:else}
            <Wave />
        {/if}
    </div>
    <div>
        <p class="title">{artworks[$currentAnimation].name}</p>
        <p class="description">{artworks[$currentAnimation].description}</p>
    </div>
</div>

<style>
    .container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        gap: 2rem;
        text-align: center;
        padding-top: 3rem;
        color: var(--text);
    }

    .projects-nav {
        padding: 2rem 1rem 0 1rem;
        margin: 0;
        display: flex;
        justify-content: space-between;
        align-items: center;
    }
    .projects-index {
        font-size: 1rem;
        margin: 0;
        text-align: center;
        color: var(--foam);
        border: 1px solid var(--muted);
        border-radius: 0.5rem;
        padding: 0.5rem 1rem;
    }
    button {
        color: var(--text);
        border: 1px solid var(--muted);
        border-radius: 0.5rem;
        padding: 0.5rem 1rem;
    }

    @media screen and (max-width: 460px) {
        .container {
            gap: 0;
            padding-top: 5px;
        }
    }

    .title {
        font-size: 1.5rem;
        font-weight: bold;
    }
    .description {
        font-size: 1rem;
        margin: auto;
        max-width: 400px;
    }
</style>
