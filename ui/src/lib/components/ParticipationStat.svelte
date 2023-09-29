<script lang="ts">
  import { fade } from "svelte/transition";
  export let name: string;
  export let value: string;
  export let ready: boolean;
</script>

<div class="flex flex-col gap-y-2">
  <span class="text-xl font-light">{name}</span>
  {#if ready}
    <span in:fade class="text-xl font-bold">{value}</span>
  {:else}
    <span class="bg-gray-300 w-16 text-lg animate-pulse text-transparent"
      >__</span
    >
  {/if}
</div>
