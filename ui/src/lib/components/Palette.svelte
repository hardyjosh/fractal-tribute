<svelte:options accessors />

<script lang="ts">
  import Shape from "$lib/components/Shape.svelte";
  import type { ShapeOptions } from "$lib/helpers";
  import { twMerge } from "tailwind-merge";
  import eyeDropperImg from "$lib/assets/eyedropper.png";
  import eraserImg from "$lib/assets/eraser.png";
  import type { BrushTool, Color } from "$lib/types";

  const colorsArr = [
    [0, 18, 25],
    [0, 95, 115],
    [10, 147, 150],
    [148, 210, 189],
    [233, 216, 166],
    [238, 155, 0],
    [202, 103, 2],
    [187, 62, 3],
    [174, 32, 18],
    [155, 34, 38],
  ];

  const colors: Color[] = colorsArr.map((c) => ({
    r: c[0],
    g: c[1],
    b: c[2],
  }));

  export let color: Color = colors[0];

  let rgb;
  export let graphic_option: ShapeOptions = 0;
  export let brushTool: BrushTool = "none";

  export const setColor = (_color) => {
    // if for some reason the new colour isn't in our list, just ignore this
    if (!colors.find((c) => colorsEqual(c, _color))) return;
    rgb = { ..._color, a: 1 };
    rgb = rgb;
    color = _color;
  };

  let pickedColor: Color = color;

  $: if (pickedColor) color = pickedColor;

  // helper for checking if colours are equal
  const colorsEqual = (a, b) => {
    console.log(a, b);
    return a.r == b.r && a.g == b.g && a.b == b.b;
  };
</script>

<div
  class="flex flex-col justify-center items-stretch rounded-lg border-2 border-black bg-primary-25 grow p-4 gap-y-2"
>
  <div class="flex flex-row mr-8 gap-x-2">
    <button
      on:click={() => {
        brushTool = "eye-dropper";
      }}
      color="none"
      class={twMerge(
        "p-2 rounded-md border-gray-400 border-2 hover:border-gray-800 cursor-pointer box-content",
        brushTool == "eye-dropper" && " border-black"
      )}><img alt="eye dropper icon" class="w-8" src={eyeDropperImg} /></button
    >
    <button
      on:click={() => {
        brushTool = "eraser";
      }}
      color="none"
      class={twMerge(
        "p-2 rounded-md border-gray-400 border-2 hover:border-gray-800 cursor-pointer box-content",
        brushTool == "eraser" && " border-black"
      )}><img alt="eye dropper icon" class="w-8" src={eraserImg} /></button
    >
  </div>
  <div class="bg-primary-50 font-semibold p-2 border border-black rounded-md">
    Colour
  </div>
  <div class="w-full flex flex-row items-start justify-start">
    <div class="grid grid-cols-5 gap-4 w-full">
      {#each colors as color}
        <button
          on:click={() => {
            pickedColor = color;
            brushTool = "none";
          }}
          class={twMerge(
            "flex flex-col items-center p-2 rounded-md border-gray-400 border h-12 hover:border-gray-800 cursor-pointer",
            colorsEqual(pickedColor, color) &&
              brushTool == "none" &&
              "border-2 border-black"
          )}
        >
          <div
            class="aspect-square w-8 h-8"
            style={`background-color: rgb(${color.r} ${color.g} ${color.b})`}
          /></button
        >
      {/each}
    </div>
  </div>
  <div class="bg-primary-50 font-semibold p-2 border border-black rounded-md">
    Pattern
  </div>
  <div class="grid grid-cols-5 gap-4">
    {#each new Array(35) as _, i}
      <button
        on:click={() => {
          graphic_option = i;
          brushTool = "none";
        }}
        class={twMerge(
          "flex flex-col items-center py-2 rounded-md border-gray-400 border h-12 hover:border-gray-800 cursor-pointer",
          graphic_option === i && brushTool == "none" && "border-2 border-black"
        )}
      >
        <Shape {color} shapeOption={i} />
      </button>
    {/each}
  </div>
</div>
