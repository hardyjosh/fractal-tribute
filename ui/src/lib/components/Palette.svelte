<svelte:options accessors />

<script lang="ts">
  import Shape from "$lib/components/Shape.svelte";
  import type { ShapeOptions } from "$lib/helpers";
  import { Button } from "flowbite-svelte";
  import ColorPicker from "svelte-awesome-color-picker";
  import { twMerge } from "tailwind-merge";
  import eyeDropperImg from "$lib/assets/eyedropper.png";

  export let color: { r: number; g: number; b: number } = {
    r: 255,
    g: 0,
    b: 0,
  };
  let rgb;
  export let graphic_option: ShapeOptions = 0;
  export let eyeDropper: boolean = false;
  export const setColor = (_color) => {
    rgb = { ..._color, a: 1 };
    rgb = rgb;
    color = _color;
  };

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
  <div class="w-full flex flex-row items-start justify-start">
    <div class="flex flex-col">
      <button
        on:click={() => {
          eyeDropper = true;
        }}
        color="none"
        class={twMerge(
          "p-2 rounded-md border-gray-400 border-2 hover:border-gray-800 cursor-pointer box-content",
          eyeDropper && " border-black"
        )}
        ><img alt="eye dropper icon" class="w-8" src={eyeDropperImg} /></button
      >
    </div>
    <ColorPicker
      bind:color={pickedColor}
      bind:rgb
      isAlpha={false}
      isOpen
      isInput={false}
    />
  </div>
  <div class="bg-primary-50 font-semibold p-2 border border-black rounded-md">
    Shape
  </div>
  <div class="grid grid-cols-5 gap-4">
    {#each new Array(19) as _, i}
      <button
        on:click={() => {
          graphic_option = i;
          eyeDropper = false;
        }}
        class={twMerge(
          "flex flex-col items-center py-2 rounded-md border-gray-400 border h-12 hover:border-gray-800 cursor-pointer",
          graphic_option === i && !eyeDropper && "border-2 border-black"
        )}
      >
        <Shape {color} shapeOption={i} />
      </button>
    {/each}
  </div>
</div>
