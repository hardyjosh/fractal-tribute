<script lang="ts">
  import CreateEvmKeyBinding from "$lib/components/CreateEvmKeyBinding.svelte";
  import CreateProfile from "$lib/components/CreateProfile.svelte";
  import { happ } from "$lib/stores";
  import { Button, Heading, Input } from "flowbite-svelte";
  import { createEventDispatcher } from "svelte";

  export let hasEvmAddress: boolean;

  const dispatch = createEventDispatcher();

  let howToPlayStep = 0;
  let name;

  const inc = () => {
    howToPlayStep++;
  };

  const completeOnboarding = async () => {
    const record = await $happ.createProfile({ name });
    dispatch("onboarding-complete");
  };
</script>

{#if howToPlayStep === 0}
  <div>
    Welcome to Fractal Tribute! A peer-to-peer fully distributed, collaborative
    artistic NFT game. This game began at 10am UTC on Wednesday, October 4th and
    ends at 3pm UTC on Friday October 6th. Note: If you are joining after the
    game starts you may want to wait 10-30 minutes until the full artwork syncs
    with all the other creator contributions before beginning to make moves.
  </div>
  <Button class="bg-fractalorange border-2 border-black" on:click={inc}
    >Next: How to Play</Button
  >
{:else if howToPlayStep === 1}
  <div>
    Creators make moves by placing pixels with colors and patterns… 20
    placements can be made in each move. NFTs are minted along the way, whenever
    a creator thinks their move is worthy of a snapshot.
  </div>
  <Button class="bg-fractalorange border-2 border-black" on:click={inc}
    >Next: About the NFTs</Button
  >
{:else if howToPlayStep === 2}
  <div>
    After a creator mints a snapshot, others can mint them as well, indicating
    which snapshots are their favorites in the series. All NFTs are created and
    saved/stored in the Holochain game app and minted using the Polygon network.
    A hosted marketplace of the NFT snapshots will be available for public
    minting at https://fractal-tribute.com.
  </div>
  <Button class="bg-fractalorange border-2 border-black" on:click={inc}
    >Next: How Claims Work</Button
  >
{:else if howToPlayStep === 3}
  <div>
    Sale of game NFTs will close 24 hours after the game ending. Following that,
    creators can return to the game to claim their fraction of NFT minting fees,
    based on contributions and relative position on the leaderboard. If you
    haven’t installed the game, you can also find your claim at
    https://fractal-tribute.com.
  </div>
  {#if !hasEvmAddress}
    <Button class="bg-fractalorange border-2 border-black" on:click={inc}
      >Next: Get Started!</Button
    >
  {:else}
    <Button
      class="bg-fractalorange border-2 border-black"
      on:click={completeOnboarding}>Ok, I'm ready to play!</Button
    >
  {/if}
{:else if howToPlayStep === 4}
  <Heading tag="h5">Welcome</Heading>
  <span>What is your name?</span>
  <span>Choose carefully, you can't change this later!</span>
  <Input type="text" bind:value={name} />
  <Button class="bg-fractalorange border-2 border-black" on:click={inc}
    >Create my Profile</Button
  >
{:else if howToPlayStep === 5}
  <CreateEvmKeyBinding on:evmKeyBindingCreated={completeOnboarding} />
{/if}
