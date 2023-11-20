<script lang="ts">
    import { showCv, you } from "$lib/stores/commonStore";
</script>

<div class="container">
    <button
        style={$showCv
            ? "background-color: var(--foam); box-shadow: 2px 2px 3px rgba(0, 0, 0, 0.2), inset 2px 2px 3px rgba(0, 0, 0, 0.2), inset -5px -5px 3px rgba(0, 0, 0, 0.2);"
            : ""}
        class="navigation"
        on:click={() => showCv.set(true)}>CV</button
    >
    <button
        style={$showCv
            ? ""
            : "background-color: var(--foam); box-shadow: 2px 2px 3px rgba(0, 0, 0, 0.2), inset 2px 2px 3px rgba(0, 0, 0, 0.2), inset -5px -5px 3px rgba(0, 0, 0, 0.2);"}
        class="navigation"
        on:click={() => showCv.set(false)}>Portfolio</button
    >
    <button
        class="navigation"
        on:click={() => $you && $you.scrollIntoView({ behavior: "smooth" })}
        >About you</button
    >
</div>

<style>
    .container {
        position: fixed;
        top: 4.85rem;
        left: 1.5rem;
        background: var(--pine);
        display: grid;
        z-index: 100;
        border-radius: 0.5rem;
        box-shadow: 3px 3px 6px rgba(0, 0, 0, 0.5);
    }
    .navigation {
        text-align: left;
        font-size: 1rem;
        font-weight: bold;
        padding: 1rem;
        border-radius: 0.5rem;
        color: var(--base);
    }
    .navigation:hover {
        background-color: var(--foam);
    }
</style>
