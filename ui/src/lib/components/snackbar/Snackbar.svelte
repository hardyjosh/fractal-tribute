<script lang="ts">
  import { Alert } from "flowbite-svelte";
  import { snackBars } from "./snackbar";
</script>

<div class="fixed bottom-0 left-0 right-0 p-4">
  {#each $snackBars as snackBar}
    <Alert type="error">{snackBar.message}</Alert>
  {/each}
</div>
