<script lang="ts">
  import { ShapeOptions } from "$lib/helpers";
  import defs from "../../../../dnas/fractal_tribute/zomes/integrity/fractal_tribute/src/defs.svg?raw";

  export let shapeOption: ShapeOptions;
  export let color: { r: number; g: number; b: number };
</script>

<!-- {@html defs} -->

<rect
  x="0"
  y="0"
  width="100"
  height="100"
  fill="rgb({color.r} {color.g} {color.b})"
  mask={`url(#m_${shapeOption + 1})`}
/>
