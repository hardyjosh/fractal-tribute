<script lang="ts">
  import { language } from "$lib/stores";
</script>

{#if $language == "en"}
  <slot />
{/if}
