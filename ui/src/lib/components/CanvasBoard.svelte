<script lang="ts">
  import { onMount } from "svelte";
  import type { Board } from "$lib/types";
  import { BOARD_SIZE, GRAPHIC_OPTIONS } from "$lib/helpers";
  //   import pattern1 from "$lib/assets/patterns/1.png";

  export let board: Board;
  export let scale: number = 1;

  let canvas: HTMLCanvasElement;
  let imagesLoaded: boolean = false;
  const TILE_SIZE = 40 * scale; // Size of each tile in pixels

  $: if (canvas && board && imagesLoaded) {
    drawBoard(canvas, board);
  }

  let imageArray: HTMLImageElement[] = [];

  onMount(() => {
    const loadImagePromises = Array.from(
      { length: GRAPHIC_OPTIONS },
      (_, i) => {
        return new Promise<HTMLImageElement>((resolve, reject) => {
          const img = new Image();
          img.src = `/patterns/${i + 1}.png`;
          img.onload = () => resolve(img);
          img.onerror = () => reject(new Error("Image load failed"));
        });
      }
    );

    Promise.all(loadImagePromises).then((images) => {
      imageArray = images;
      imagesLoaded = true;
    });
  });

  function drawBoard(canvas: HTMLCanvasElement, board: Board) {
    const ctx = canvas.getContext("2d");
    canvas.width = BOARD_SIZE * TILE_SIZE;
    canvas.height = BOARD_SIZE * TILE_SIZE;

    // Create off-screen canvas for each group
    const offscreenCanvas = document.createElement("canvas");
    offscreenCanvas.width = canvas.width;
    offscreenCanvas.height = canvas.height;
    const offscreenCtx = offscreenCanvas.getContext("2d");

    const groups: Array<any[]> = Array.from(
      { length: GRAPHIC_OPTIONS * 2 + 1 },
      () => []
    );

    // Step 1: Group tiles
    board.forEach((row, x) => {
      row.forEach((tile, y) => {
        if (tile.color && tile.graphic_option !== null) {
          const base_x = x * TILE_SIZE;
          const base_y = y * TILE_SIZE;
          const fill = `rgb(${tile.color.r}, ${tile.color.g}, ${tile.color.b})`;

          const groupIndex = tile.graphic_option;
          groups[groupIndex].push({
            fill,
            x: base_x,
            y: base_y,
          });
        }
      });
    });

    // Step 2 and 3: Draw Grouped Tiles and Apply Mask
    groups.forEach((group, index) => {
      offscreenCtx.clearRect(
        0,
        0,
        offscreenCanvas.width,
        offscreenCanvas.height
      );
      const maskImage = imageArray[index % GRAPHIC_OPTIONS];

      if (index < GRAPHIC_OPTIONS) {
        // Draw rectangles in the group on offscreen canvas
        group.forEach(({ fill, x, y }) => {
          offscreenCtx.fillStyle = fill;
          offscreenCtx.fillRect(x, y, TILE_SIZE, TILE_SIZE);
        });

        // Set composite operation
        offscreenCtx.globalCompositeOperation = "destination-in";

        // Apply mask (use your actual mask image here)
        //   maskImage.src = `path/to/mask_${index}.png`;
        //   maskImage.onload = () => {
        offscreenCtx.drawImage(
          maskImage,
          0,
          0,
          offscreenCanvas.width,
          offscreenCanvas.height
        );

        // Reset composite operation and draw back onto the main canvas
        offscreenCtx.globalCompositeOperation = "source-over";
      } else if (index < GRAPHIC_OPTIONS * 2) {
        // Inverted masking technique
        offscreenCtx.globalCompositeOperation = "source-over";

        group.forEach(({ fill, x, y }) => {
          offscreenCtx.fillStyle = fill;
          offscreenCtx.fillRect(x, y, TILE_SIZE, TILE_SIZE);
        });

        offscreenCtx.save();
        group.forEach(({ x, y }) => {
          offscreenCtx.rect(x, y, TILE_SIZE, TILE_SIZE);
        });
        offscreenCtx.clip();

        offscreenCtx.globalCompositeOperation = "source-over";
        offscreenCtx.drawImage(maskImage, 0, 0, canvas.width, canvas.height);
        offscreenCtx.restore();
      } else {
        // Just colored square, no mask
        group.forEach(({ fill, x, y }) => {
          ctx.fillStyle = fill;
          ctx.fillRect(x, y, TILE_SIZE, TILE_SIZE);
        });
      }

      ctx.drawImage(offscreenCanvas, 0, 0);
    });
  }
</script>

<div>
  <canvas bind:this={canvas} class="w-full h-full" />
</div>
