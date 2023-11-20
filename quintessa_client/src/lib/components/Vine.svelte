<script lang="ts">
    import { theme } from "$lib/stores/commonStore";
    import { device } from "$lib/utilities/device";
    import P5, { type Sketch } from "p5-svelte";

    const sketch: Sketch = (p) => {
        let x: number;
        let y: number;

        let g: number;
        let b: number;

        p.setup = () => {
            if ($device === "mobile") {
                p.createCanvas(300, 600);
            } else {
                p.createCanvas(600, 600);
            }

            x = p.width / 2;
            y = p.height;

            g = p.random(255);
            b = p.random(g);

            $theme === "light"
                ? p.background(255, 250, 243)
                : p.background(31, 29, 46);
            p.noSmooth();
        };

        p.draw = () => {
            x += p.random(-1, 1);
            y -= p.random(1);

            if (x < 0) x = p.width;
            if (x > p.width) x = 0;

            if (y < 0) y = p.height;

            g += p.random(-10, 10);
            g = p.constrain(g, 0, 255);

            b += p.random(-10, 10);
            b = p.constrain(b, 0, g);

            p.stroke(0, g, b);
            p.point(x, y);
        };
    };
</script>

<P5 {sketch} />
