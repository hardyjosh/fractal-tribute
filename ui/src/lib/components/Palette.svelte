<svelte:options accessors />

<script lang="ts">
  import Shape from "$lib/components/Shape.svelte";
  import type { ShapeOptions } from "$lib/helpers";
  import { twMerge } from "tailwind-merge";
  import eyeDropperImg from "$lib/assets/eyedropper.png";
  import eraserImg from "$lib/assets/eraser.png";
  import type { BrushTool, Color } from "$lib/types";
  import {
    EyeOutline,
    EyeSlashOutline,
    ArrowsRepeatOutline,
  } from "flowbite-svelte-icons";
  import { onMount } from "svelte";
  import { isHotHolder } from "$lib/stores/hotHolder";
  import En from "$lib/components/i18n/En.svelte";
  import Tr from "$lib/components/i18n/Tr.svelte";

  // const colorsArr = [
  //   [0, 18, 25],
  //   [0, 95, 115],
  //   [10, 147, 150],
  //   [148, 210, 189],
  //   [233, 216, 166],
  //   [238, 155, 0],
  //   [202, 103, 2],
  //   [187, 62, 3],
  //   [174, 32, 18],
  //   [155, 34, 38],
  // ];

  const baseColors = [
    [58, 34, 32],
    [158, 59, 18],
    [5, 98, 103],
    [23, 56, 75],
    [212, 73, 70],
    [229, 91, 19],
    [13, 65, 14],
    [120, 135, 32],
    [134, 26, 58],
    [255, 39, 104],
  ];

  const bonusColors = [
    [232, 153, 0],
    [139, 44, 206],
  ];

  let colorsArr = baseColors;

  let colors = colorsArr.map((c) => ({
    r: c[0],
    g: c[1],
    b: c[2],
  })) as Color[];

  $: colorsArr = $isHotHolder ? baseColors.concat(bonusColors) : baseColors;
  $: colors = colorsArr.map((c) => ({
    r: c[0],
    g: c[1],
    b: c[2],
  })) as Color[];

  export let color: Color = colors[0];

  let rgb;

  export let graphic_option: ShapeOptions = 0;
  export let brushTool: BrushTool = "none";
  export let hideGrid: boolean = false;

  export let refreshBoard: () => void;
  export let refreshingBoard: boolean;

  export const setColor = (_color) => {
    // if for some reason the new colour isn't in our list, just ignore this
    if (!colors.find((c) => colorsEqual(c, _color))) return;
    rgb = { ..._color, a: 1 };
    rgb = rgb;
    color = _color;
    pickedColor = _color;
  };

  export const setGraphicOption = (option) => {
    console.log("setting graphic option");
    graphic_option = option;
    brushTool = "none";
  };

  let pickedColor: Color = color;

  $: if (pickedColor) color = pickedColor;

  // helper for checking if colours are equal
  const colorsEqual = (a, b) => {
    return a.r == b.r && a.g == b.g && a.b == b.b;
  };
</script>

<div
  class="flex flex-col justify-center items-stretch rounded-lg border-2 border-black bg-primary-25 grow p-4 gap-y-2"
>
  <div class="flex flex-row gap-x-2 w-full justify-between">
    <div class="flex gap-x-2">
      <button
        on:click={() => {
          brushTool = "eye-dropper";
        }}
        color="none"
        class={twMerge(
          "p-2 rounded-md border-gray-400 border-2 hover:border-gray-800 cursor-pointer box-content",
          brushTool == "eye-dropper" && " border-black"
        )}
        ><img alt="eye dropper icon" class="w-8" src={eyeDropperImg} /></button
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
    <div class="flex gap-x-2">
      <button
        class="w-32 ml-auto flex gap-x-2 items-center p-2 rounded-md border-gray-400 border-2 hover:border-gray-800 cursor-pointer box-content justify-self-end"
        on:click={() => (hideGrid = !hideGrid)}
      >
        {#if !hideGrid}
          <EyeSlashOutline class="w-8" />
          <En>Hide grid</En><Tr>Izgarayı gizle</Tr>
        {:else}
          <EyeOutline class="w-8" />
          Show grid
        {/if}
      </button>
      <button
        class="w-40 ml-auto flex gap-x-2 items-center p-2 rounded-md border-gray-400 border-2 hover:border-gray-800 cursor-pointer box-content justify-self-end"
        on:click={refreshBoard}
      >
        <ArrowsRepeatOutline class="w-8" />
        {#if refreshingBoard}
          Refreshing...
        {:else}
          <En>Refresh board</En><Tr>Panoyu tazele</Tr>
        {/if}
      </button>
    </div>
  </div>
  <div class="bg-primary-50 font-semibold p-2 border border-black rounded-md">
    <En>Colour</En><Tr>Renk</Tr>
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
            colorsEqual(pickedColor, color) && "border-2 border-black"
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
    <En>Pattern</En><Tr>Desen</Tr>
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
