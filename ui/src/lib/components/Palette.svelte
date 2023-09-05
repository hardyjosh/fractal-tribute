<script lang="ts">
  import Shape from "$lib/components/Shape.svelte";
  import type { ShapeOptions } from "$lib/helpers";
  import ColorPicker from "svelte-awesome-color-picker";
  import { twMerge } from "tailwind-merge";

  export let color: { r: number; g: number; b: number } = {
    r: 255,
    g: 0,
    b: 0,
  };
  export let graphic_option: ShapeOptions = 0;

  let pickedColor;

  $: if (pickedColor)
    color = {
      r: Math.floor(pickedColor.rgba.r),
      g: Math.floor(pickedColor.rgba.g),
      b: Math.floor(pickedColor.rgba.b),
    };
</script>

<div
  class="flex flex-col justify-center items-stretch rounded-lg border-2 border-black bg-primary-25 grow p-4 gap-y-2"
>
  <div class="bg-primary-50 font-semibold p-2 border border-black rounded-md">
    Colour
  </div>
  <div class="w-full flex flex-col items-center">
    <ColorPicker
      bind:color={pickedColor}
      isAlpha={false}
      isOpen
      isInput={false}
    />
  </div>
  <div class="bg-primary-50 font-semibold p-2 border border-black rounded-md">
    Shape
  </div>
  <div class="grid grid-cols-5 gap-4">
    {#each new Array(11) as _, i}
      <button
        on:click={() => {
          graphic_option = i;
        }}
        class={twMerge(
          "flex flex-col items-center py-2 rounded-md border-gray-400 border h-12 hover:border-gray-800 cursor-pointer",
          graphic_option === i && "border-2 border-black"
        )}
      >
        <Shape {color} shapeOption={i} />
      </button>
    {/each}
  </div>
</div>
