<script lang="ts">
  import { happ } from "$lib/stores";
  import { Button, Input } from "flowbite-svelte";
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  export let name: string;

  const createProfile = async () => {
    dispatch("profile-created", { name });
  };
</script>

<div class="flex flex-col gap-y-4 items-start">
  <span>What is your name?</span>
  <Input type="text" bind:value={name} />
  <Button
    class="bg-fractalorange border-2 border-black"
    on:click={createProfile}>Create my Profile</Button
  >
</div>
