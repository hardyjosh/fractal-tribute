<script lang="ts">
  import { language } from "$lib/stores";
</script>

{#if $language == "tr"}
  <slot />
{/if}
