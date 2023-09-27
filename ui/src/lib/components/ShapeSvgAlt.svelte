<script lang="ts">
  import { GRAPHIC_OPTIONS } from "$lib/helpers";

  export let shapeOption;
  export let color: { r: number; g: number; b: number };
  export let x: number = 0;
  export let y: number = 0;
</script>

{#if shapeOption == GRAPHIC_OPTIONS * 2}
  <rect
    {x}
    {y}
    width="100"
    height="100"
    fill="rgb({color.r} {color.g} {color.b})"
  />
{:else if shapeOption < GRAPHIC_OPTIONS}
  <rect x="0" y="0" width="100" height="100" fill="white" />
  <rect
    {x}
    {y}
    width="100"
    height="100"
    fill="rgb({color.r} {color.g} {color.b})"
    mask={`url(#m_${(shapeOption % GRAPHIC_OPTIONS) + 1})`}
  />
{:else}
  <rect
    {x}
    {y}
    width="100"
    height="100"
    fill="rgb({color.r} {color.g} {color.b})"
  />
  <rect
    {x}
    {y}
    width="100"
    height="100"
    fill="white"
    mask={`url(#m_${(shapeOption % GRAPHIC_OPTIONS) + 1})`}
  />
{/if}
