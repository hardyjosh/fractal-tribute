<script lang="ts">
  import { ShapeOptions } from "$lib/helpers";

  export let shapeOption: ShapeOptions;
  export let color: { r: number; g: number; b: number };
</script>

<svg class="w-full" viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <clipPath id="clip">
      <rect x="0" y="0" width="100" height="100" />
    </clipPath>
  </defs>
  <g clip-path="url(#clip)">
    {#if shapeOption === ShapeOptions.Square}
      <rect
        x="0"
        y="0"
        width="100"
        height="100"
        fill="rgb({color.r} {color.g} {color.b})"
      />
    {/if}
    {#if shapeOption === ShapeOptions.LeftRectangle}
      <rect
        x="0"
        y="0"
        width="50"
        height="100"
        fill="rgb({color.r} {color.g} {color.b})"
      />
    {/if}
    {#if shapeOption === ShapeOptions.UpperRectangle}
      <rect
        x="0"
        y="0"
        width="100"
        height="50"
        fill="rgb({color.r} {color.g} {color.b})"
      />
    {/if}
    {#if shapeOption === ShapeOptions.RightRectangle}
      <rect
        x="50"
        y="0"
        width="50"
        height="100"
        fill="rgb({color.r} {color.g} {color.b})"
      />
    {/if}
    {#if shapeOption === ShapeOptions.LowerRectangle}
      <rect
        x="0"
        y="50"
        width="100"
        height="50"
        fill="rgb({color.r} {color.g} {color.b})"
      />
    {/if}
    {#if shapeOption === ShapeOptions.UpperLeftTriangle}
      <polygon
        points="0,0 100,0 0,100"
        fill="rgb({color.r} {color.g} {color.b})"
      />
    {/if}
    {#if shapeOption === ShapeOptions.UpperRightTriangle}
      <polygon
        points="0,0 100,0 100,100"
        fill="rgb({color.r} {color.g} {color.b})"
      />
    {/if}
    {#if shapeOption === ShapeOptions.LowerRightTriangle}
      <polygon
        points="0,0 100,100 0,100"
        fill="rgb({color.r} {color.g} {color.b})"
      />
    {/if}
    {#if shapeOption === ShapeOptions.LowerLeftTriangle}
      <polygon
        points="0,0 100,100 0,100"
        fill="rgb({color.r} {color.g} {color.b})"
      />
    {/if}
    {#if shapeOption === ShapeOptions.Circle}
      <circle
        cx="50"
        cy="50"
        r="50"
        fill="rgb({color.r} {color.g} {color.b})"
      />
    {/if}
    {#if shapeOption === ShapeOptions.Diamond}
      <polygon
        points="0,50 50,0 100,50 50,100"
        fill="rgb({color.r} {color.g} {color.b})"
      />
    {/if}
  </g>
</svg>
