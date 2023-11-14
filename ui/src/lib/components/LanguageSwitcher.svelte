<script lang="ts">
  import { language } from "$lib/stores";
</script>

<div class="flex gap-x-1">
  <button
    on:click={() => {
      $language = "tr";
    }}
    class="language"
    class:active={$language == "tr"}>🇹🇷</button
  >
  <button
    on:click={() => {
      $language = "en";
    }}
    class="language"
    class:active={$language == "en"}>🇬🇧</button
  >
</div>

<style lang="postcss">
  .active {
    @apply !border-gray-700;
  }

  .language {
    @apply border rounded-full p-1 leading-none aspect-square h-8 w-8 border-transparent;
  }
</style>
