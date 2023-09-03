<script lang="ts">
  import { Toast } from "flowbite-svelte";
  import {
    CheckCircleSolid,
    ExclamationCircleSolid,
    FireOutline,
    CloseCircleSolid,
  } from "flowbite-svelte-icons";
  import { fade } from "svelte/transition";
  import type { Writable } from "svelte/store";

  export let toasts: Writable<any>;
</script>

<div class="fixed bottom-0 left-0 right-0 p-4">
  {#if $toasts}
    {#each $toasts as toast}
      {#if toast.type == "success"}
        <Toast color="green" transition={fade}>
          <svelte:fragment slot="icon">
            <CheckCircleSolid class="w-5 h-5" />
            <span class="sr-only">Check icon</span>
          </svelte:fragment>
          {toast.message}
        </Toast>
      {/if}
      {#if toast.type == "error"}
        <Toast color="red" transition={fade}>
          <svelte:fragment slot="icon">
            <ExclamationCircleSolid class="w-5 h-5" />
            <span class="sr-only">Exclamation icon</span>
          </svelte:fragment>
          {toast.message}
        </Toast>
      {/if}
      {#if toast.type == "info"}
        <Toast color="blue" transition={fade}>
          <svelte:fragment slot="icon">
            <CloseCircleSolid class="w-5 h-5" />
            <span class="sr-only">Close icon</span>
          </svelte:fragment>
          {toast.message}
        </Toast>
      {/if}
    {/each}
  {/if}
</div>
