<script lang="ts">
    export let width: number;
</script>

<div class="loader" style={`width:${width}rem; height:${width}rem`} />

<style>
    .loader {
        border: 5px solid var(--foam);
        border-radius: 50%;
        border-top: 5px solid var(--base);
        -webkit-animation: spin 2s linear infinite; /* Safari */
        animation: spin 2s linear infinite;
        /* center both vertically and horizontally */
        position: absolute;
        left: 50%;
        top: 50%;
        transform: translate(-50%, -50%);
    }

    /* Safari */
    @-webkit-keyframes spin {
        0% {
            -webkit-transform: rotate(0deg);
        }
        100% {
            -webkit-transform: rotate(360deg);
        }
    }

    @keyframes spin {
        0% {
            transform: rotate(0deg);
        }
        100% {
            transform: rotate(360deg);
        }
    }
</style>
