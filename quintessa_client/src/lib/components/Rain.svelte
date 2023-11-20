<script lang="ts">
    import { theme } from "$lib/stores/commonStore";
    import { device } from "$lib/utilities/device";
    import P5, { type Sketch } from "p5-svelte";

    const sketch: Sketch = (p) => {
        let smallPoint: number;
        let largePoint: number;

        p.setup = () => {
            if ($device === "mobile") {
                p.createCanvas(300, 600);
            } else {
                p.createCanvas(600, 600);
            }

            smallPoint = 3;
            largePoint = 10;

            p.noStroke();
            $theme === "light"
                ? p.background(255, 250, 243)
                : p.background(31, 29, 46);
        };

        p.draw = () => {
            let pointillize = p.map(
                p.mouseX,
                0,
                p.width,
                smallPoint,
                largePoint
            );

            let x = p.floor(p.random(p.width));
            let y = p.floor(p.random(p.height));

            $theme === "light"
                ? p.fill(87, 82, 121, p.mouseY)
                : p.fill(224, 222, 244, p.mouseY);
            // p.fill(156, 207, 216, p.mouseY);
            p.ellipse(x, y, pointillize, pointillize);
        };
    };
</script>

<P5 {sketch} />
