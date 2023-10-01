<script lang="ts">
  import { fade } from "svelte/transition";
  import CreateEvmKeyBinding from "$lib/components/CreateEvmKeyBinding.svelte";
  import { happ } from "$lib/stores";
  import { A, Button, Heading, Input } from "flowbite-svelte";
  import { createEventDispatcher } from "svelte";
  import singlelogo from "$lib/assets/single-logo.svg";
  import mintsnapshot from "$lib/assets/mint-snapshot.svg";
  import howtoplay1 from "$lib/assets/how-to-play-1.svg";
  import howtoplay2 from "$lib/assets/how-to-play-2.svg";
  import howtoplay4 from "$lib/assets/how-to-play-4.svg";

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

<div
  class="flex flex-col justify-between items-center gap-y-6 max-w-[650px] min-h-[500px] p-6"
>
  {#if howToPlayStep === 0}
    <div in:fade class="flex flex-col gap-y-5 text-center items-center">
      <img class="w-32" src={singlelogo} alt="fractal tribute logo" />
      <Heading tag="h2">Welcome to Fractal Tribute!</Heading>
      <Heading tag="h6" class="font-semibold"
        >A peer-to-peer fully distributed, collaborative artistic NFT game.</Heading
      >
      <p>
        This game began at 10am UTC on Wednesday, October 4th and ends at 3pm
        UTC on Friday October 6th.
      </p>
      <p>
        Note: If you are joining after the game starts you may want to wait
        10-30 minutes until the full artwork syncs with all the other creator
        contributions before beginning to make moves.
      </p>
    </div>
    <Button class="bg-fractalorange border-2 border-black" on:click={inc}
      >Next: How to Play</Button
    >
  {:else if howToPlayStep === 1}
    <div in:fade class="flex flex-col gap-y-5 text-center items-center">
      <img class="w-80" src={howtoplay1} alt="" />
      <p>
        Creators make moves by placing pixels with colors and patterns… 20
        placements can be made in each move.
      </p>
      <p>
        NFTs are minted along the way, whenever a creator thinks their move is
        worthy of a snapshot.
      </p>
    </div>
    <Button class="bg-fractalorange border-2 border-black" on:click={inc}
      >Next: About the NFTs</Button
    >
  {:else if howToPlayStep === 2}
    <div in:fade class="flex flex-col gap-y-5 text-center items-center">
      <img class="w-80" src={mintsnapshot} alt="mint snapshot" />
      <p>
        After a creator mints a snapshot, others can mint them as well,
        indicating which snapshots are their favorites in the series.
      </p>
      <p>
        All NFTs are created and saved/stored in the Holochain game app and
        minted on the Polygon network. A hosted marketplace of the NFT snapshots
        will be available for public minting at <A
          href="https://fractal-tribute.com">fractal-tribute.com</A
        >.
      </p>
    </div>
    <Button class="bg-fractalorange border-2 border-black" on:click={inc}
      >Next: How Claims Work</Button
    >
  {:else if howToPlayStep === 3}
    <div in:fade class="flex flex-col gap-y-5 text-center items-center">
      <img class="w-80" src={howtoplay4} alt="" />
      <p>
        Sale of game NFTs will close 24 hours after the game ending. Following
        that, creators can return to the game to claim their fraction of NFT
        minting fees, based on contributions and relative position on the
        leaderboard.
      </p>
      <p>
        If you haven’t installed the game, you can also find your claim at <A
          href="https://fractal-tribute.com">fractal-tribute.com</A
        >.
      </p>
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
    <div in:fade class="flex flex-col gap-y-5 text-center">
      <Heading tag="h4">Ok, let's get you set up to play.</Heading>
    </div>
    <div class="flex flex-col gap-y-5 text-center">
      <Heading tag="h5">What is your name?</Heading>
      <span>Choose carefully, you can't change this later!</span>
      <Input defaultClass="text-center" type="text" bind:value={name} />
    </div>
    <Button
      disabled={!name}
      class="bg-fractalorange border-2 border-black"
      on:click={inc}>Create my Profile</Button
    >
  {:else if howToPlayStep === 5}
    <CreateEvmKeyBinding on:evmKeyBindingCreated={inc} />
  {:else if howToPlayStep === 6}
    <div in:fade class="flex flex-col gap-y-5 text-center">
      <Heading tag="h4">That's it! Be creative and have fun 💫</Heading>
      <p>
        To see these instructions again, click on "how to play" on the top
        right.
      </p>
    </div>
    <Button
      class="bg-fractalorange border-2 border-black"
      on:click={completeOnboarding}>I'm ready!</Button
    >
  {/if}
</div>
