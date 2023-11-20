<script lang="ts">
    import { theme } from "$lib/stores/commonStore";
    import { device } from "$lib/utilities/device";
    import P5, { type Sketch } from "p5-svelte";

    const sketch: Sketch = (p5) => {
        let t = 0; // time variable

        p5.setup = () => {
            if ($device === "mobile") {
                p5.createCanvas(300, 600);
            } else {
                p5.createCanvas(600, 600);
            }
            p5.noStroke();
            $theme === "light" ? p5.fill("#575279") : p5.fill("#e0def4");
        };

        p5.draw = () => {
            // translucent background (creates trails)
            $theme === "light"
                ? p5.background(255, 250, 243, 10)
                : p5.background(31, 29, 46, 10);

            // make a x and y grid of ellipses
            for (let x = 0; x <= p5.width; x = x + 50) {
                for (let y = 0; y <= p5.height; y = y + 50) {
                    // starting point of each circle depends on mouse position
                    const xAngle = p5.map(
                        p5.mouseX,
                        0,
                        p5.width,
                        -4 * p5.PI,
                        4 * p5.PI,
                        true
                    );
                    const yAngle = p5.map(
                        p5.mouseY,
                        0,
                        p5.height,
                        -4 * p5.PI,
                        4 * p5.PI,
                        true
                    );

                    // and also varies based on the particle's location
                    const angle =
                        xAngle * (x / p5.width) + yAngle * (y / p5.height);

                    // each particle moves in a circle
                    const myX = x + 40 * p5.cos(2 * p5.PI * t + angle);
                    const myY = y + 40 * p5.sin(2 * p5.PI * t + angle);

                    p5.ellipse(myX, myY, 3); // draw particle
                }
            }

            t = t + 0.01; // update time
        };
    };
</script>

<P5 {sketch} />
