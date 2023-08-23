<script lang="ts">
  import ColorPicker from "svelte-awesome-color-picker";

  export let color: { r: number; g: number; b: number } = {
    r: 256,
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

<ColorPicker bind:color={pickedColor} isAlpha={false} isOpen isInput={false} />
