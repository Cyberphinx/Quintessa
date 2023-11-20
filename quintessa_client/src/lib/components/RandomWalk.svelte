<script lang="ts">
    import { theme } from "$lib/stores/commonStore";
    import { device } from "$lib/utilities/device";
    import P5, { type Sketch } from "p5-svelte";

    const sketch: Sketch = (p) => {
        let x: number[] = [];
        let y: number[] = [];
        let r: number[] = [];
        let g: number[] = [];
        let b: number[] = [];

        p.setup = () => {
            if ($device === "mobile") {
                p.createCanvas(300, 600);
            } else {
                p.createCanvas(600, 600);
            }

            p.noSmooth();
            $theme === "light"
                ? p.background(255, 250, 243)
                : p.background(31, 29, 46);
            p.frameRate(1000);
        };

        p.draw = () => {
            for (let i = 0; i < x.length; i++) {
                x[i] += p.random(-3, 5);
                y[i] += p.random(-5, 3);

                if (x[i] < 0) {
                    x[i] = p.width;
                }
                if (x[i] > p.width) {
                    x[i] = 0;
                }

                if (y[i] < 0) {
                    y[i] = p.height;
                }
                if (y[i] > p.height) {
                    y[i] = 0;
                }

                p.stroke(r[i], g[i], b[i]);
                p.point(x[i], y[i]);
            }
        };

        p.mousePressed = () => {
            x = p.append(x, p.mouseX);
            y = p.append(y, p.mouseY);
            r = p.append(r, p.random(256));
            g = p.append(g, p.random(256));
            b = p.append(b, p.random(256));
        };
    };
</script>

<P5 {sketch} />
