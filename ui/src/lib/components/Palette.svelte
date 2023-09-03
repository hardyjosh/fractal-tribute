<script lang="ts">
  import ColorPicker from "svelte-awesome-color-picker";

  export let color: { r: number; g: number; b: number } = {
    r: 255,
    g: 0,
    b: 0,
  };
  export let graphic_option: number;

  let pickedColor;

  $: if (pickedColor)
    color = {
      r: Math.floor(pickedColor.rgba.r),
      g: Math.floor(pickedColor.rgba.g),
      b: Math.floor(pickedColor.rgba.b),
    };
</script>

<div
  class="flex flex-col justify-center items-stretch rounded-lg border-2 border-black bg-[#FFFBFA] grow p-4 gap-y-2"
>
  <div class="bg-[#FAF2F1] font-semibold p-2 border border-black rounded-md">
    Colour
  </div>
  <ColorPicker
    bind:color={pickedColor}
    isAlpha={false}
    isOpen
    isInput={false}
  />
  <div class="bg-[#FAF2F1] font-semibold p-2 border border-black rounded-md">
    Shape
  </div>
</div>
