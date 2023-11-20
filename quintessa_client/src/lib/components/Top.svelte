<script lang="ts">
    import { goto } from "$app/navigation";
</script>

<button class="top" on:click={() => goto("/")}
    ><svg
        xmlns="http://www.w3.org/2000/svg"
        width="24"
        height="24"
        viewBox="0 0 24 24"
        fill="none"
        stroke="var(--base)"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        class="lucide lucide-chevron-up"><path d="m18 15-6-6-6 6" /></svg
    ></button
>

<style>
    .top {
        position: fixed;
        bottom: 2rem;
        right: 2rem;
        background-color: var(--love);
        color: var(--base);
        border-radius: 50%;
        width: 40px;
        height: 40px;
        z-index: 100;
    }

    @media screen and (max-width: 660px) {
        .top {
            bottom: 3rem;
            right: 1rem;
        }
    }
</style>

