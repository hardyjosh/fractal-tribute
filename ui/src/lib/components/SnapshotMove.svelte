<script lang="ts">
  import { fade } from "svelte/transition";
  import { onMount, createEventDispatcher } from "svelte";
  import { account, network } from "svelte-wagmi-stores";
  import { type ActionHash } from "@holochain/client";
  import {
    happ,
    nftContract,
    paymentTokenAddress,
    web3modal,
  } from "$lib/stores";
  import { Button, Spinner, Heading, Alert } from "flowbite-svelte";
  import {
    hexToBigInt,
    keccak256,
    type Address,
    parseEther,
    type Hex,
  } from "viem";
  import { snapshotEvaluable } from "$lib/helpers";
  import { waitForTransaction } from "@wagmi/core";

  const dispatch = createEventDispatcher();

  export let isPostMove: boolean = false;
  export let open: boolean;

  export let move: ActionHash;
  $: _move = hexToBigInt(keccak256(move));

  const price = parseEther("0");

  let key: Address;
  $: mismatchingKey = $account?.address && key && $account?.address !== key;
  $: wrongNetwork =
    $account?.address && $happ.dnaProperties.chainId !== $network?.chain?.id;

  onMount(async () => {
    key = await $happ.getEvmAddress();
  });

  let hash: Hex;

  $: ({ write, status, error } = $nftContract.write({
    functionName: "flow",
    args: [snapshotEvaluable, [_move], []],
    onSuccess: ({ hash: _hash }) => {
      hash = _hash;
      waitForTransaction({ hash: _hash, confirmations: 5 }).then(() => {
        dispatch("snapshotMinted");
      });
    },
  }));

  const snapshotMove = async () => {
    await $happ.createTokenIdForGameMove(move);
    await write();
  };
</script>

{#if $status == "idle" || $status == "error"}
  <div in:fade class="flex flex-col justify-center gap-y-4">
    {#if isPostMove}
      <Heading tag="h4">Nice move!</Heading>
      <Heading tag="h5">Would you like to mint a snapshot?</Heading>
    {:else}
      <Heading tag="h4">Create snapshot</Heading>
    {/if}
    <p>Creating a snapshot is free, you'll just need to pay the gas fee.</p>
    <p>
      One you have created your snapshot, other players (and the public) will be
      able to mint your snapshot to push it up the leaderboard.
    </p>
    <p>
      The MATIC collected will be sent to the game pool to be redistributed to
      players at the end of the game
    </p>
    <div class="flex gap-x-2">
      <Button
        class="border-2 border-black"
        color="none"
        on:click={() => {
          open = false;
        }}>Maybe later</Button
      >
      {#if $account?.isConnected}
        <Button
          class="bg-fractalorange border-2 border-black"
          disabled={mismatchingKey || wrongNetwork}
          on:click={snapshotMove}
        >
          Mint snapshot
        </Button>
      {:else}
        <Button
          class="bg-fractalorange border-2 border-black self-start"
          on:click={() => {
            $web3modal.openModal();
          }}>Connect wallet</Button
        >
      {/if}
    </div>
    {#if $status == "error"}
      <p transition:fade class="text-red-500">
        {$error?.details || $error?.shortMessage || $error}
      </p>
    {/if}
    {#if wrongNetwork}
      <Alert>
        <p>
          You're connected to the wrong network. Please switch to the{" "}
          {$happ.dnaProperties.chainId === 137 ? "Polygon" : "Mumbai"}{" "}
          network.
        </p>
      </Alert>
    {:else if mismatchingKey}
      <Alert>
        <p>
          You previously bound the Ethereum wallet {key} to your Holochain agent
          key.
        </p>
        <p>
          You'll need switch to this account in your wallet before you can mint.
        </p>
      </Alert>
    {/if}
  </div>
{:else if $status === "loading"}
  <div in:fade class="flex flex-col items-center gap-y-4 my-12">
    <Spinner size="10" class="mr-2" />
    <Heading tag="h4" class="text-center">Minting snapshot</Heading>
    <span>Please check your wallet to confirm</span>
  </div>
{:else if $status === "success"}
  <div in:fade class="flex flex-col items-center gap-y-4 my-12">
    <Heading class="text-center" tag="h4">Snapshot minted!</Heading>
    <a
      href={`${$network.chain.blockExplorers.default.url}/tx/${hash}`}
      target="_blank"
      class="underline"
    >
      View on explorer</a
    >
    <Button
      class="bg-fractalorange border-2 border-black"
      on:click={() => {
        open = false;
      }}>Done</Button
    >
  </div>
{/if}
