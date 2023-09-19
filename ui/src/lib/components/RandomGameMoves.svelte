<script lang="ts">
  import { generateRandomGameMove } from "$lib/helpers";
  import { happ } from "$lib/stores";

  let interval;

  const startMakingRandomMoves = () => {
    interval = setInterval(() => {
      $happ.createGameMove(generateRandomGameMove());
    }, 100);
  };

  const stopMakingRandomMoves = () => {
    clearInterval(interval);
    interval = null;
  };
</script>

{#if !interval}
  <button on:click={startMakingRandomMoves}>Start making random moves</button>
{:else}
  <button on:click={stopMakingRandomMoves}>Stop making random moves</button>
{/if}
