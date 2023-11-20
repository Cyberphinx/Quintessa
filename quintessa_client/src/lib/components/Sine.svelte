<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { browser } from "$app/environment";
    import { theme } from "$lib/stores/commonStore";

    let p5Instance: any;
    let container: HTMLElement;

    if (browser) {
        onMount(async () => {
            const p5 = (await import("p5")).default;

            p5Instance = new p5((p) => {
                let xspacing: number = 20; // How far apart should each horizontal location be spaced
                let w: number; // Width of entire wave
                let theta: number = 0.0; // Start angle at 0
                let amplitude: number = 100.0; // Height of wave
                let period: number = 500.0; // How many pixels before the wave repeats
                let dx: number; // Value for incrementing X, a function of period and xspacing
                let yvalues: number[]; // Using an array to store height values for the wave

                p.setup = function () {
                    p.createCanvas(600, 600);
                    w = p.width + 10;
                    dx = (p.TWO_PI / period) * xspacing;
                    yvalues = new Array(Math.floor(w / xspacing)).fill(0);
                };

                p.draw = function () {
                    $theme === "light"
                        ? p.background(255, 250, 243)
                        : p.background(31, 29, 46);
                    calcWave();
                    renderWave();
                };

                function calcWave() {
                    // Increment theta (try different values for 'angular velocity' here
                    theta += 0.03;

                    // For every x value, calculate a y value with sine function
                    let x: number = theta;
                    for (let i = 0; i < yvalues.length; i++) {
                        yvalues[i] = p.sin(x) * amplitude;
                        x += dx;
                    }
                }

                function renderWave() {
                    p.noStroke();
                    $theme === "light"
                        ? p.fill(87, 82, 121)
                        : p.fill(224, 222, 244);

                    // A simple way to draw the wave with an ellipse at each location
                    for (let x = 0; x < yvalues.length; x++) {
                        p.ellipse(
                            x * xspacing,
                            p.height / 2 + yvalues[x],
                            yvalues[x] / 15 + 15,
                            yvalues[x] / 15 + 15
                        );
                    }
                }
            }, container);
        });

        onDestroy(() => {
            p5Instance.remove();
        });
    }
</script>

<div id="canvas-container" bind:this={container} />
