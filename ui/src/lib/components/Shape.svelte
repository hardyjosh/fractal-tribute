<script lang="ts">
  import ShapeSvg from "$lib/components/ShapeSvg.svelte";
  import { ShapeOptions } from "$lib/helpers";

  export let shapeOption: ShapeOptions;
  export let color: { r: number; g: number; b: number };
</script>

<svg class="w-full" viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
  <ShapeSvg {color} {shapeOption} />
</svg>
