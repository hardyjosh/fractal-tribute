<script lang="ts">
  import { onMount, createEventDispatcher } from "svelte";
  import addresses from "$lib/addresses.json";
  import { account } from "svelte-wagmi-stores";
  import { type ActionHash } from "@holochain/client";
  import { nftContract, paymentToken, paymentTokenAddress } from "$lib/stores";
  import { Button, Spinner, Heading } from "flowbite-svelte";
  import {
    hexToBigInt,
    keccak256,
    type Address,
    parseEther,
    formatUnits,
  } from "viem";
  import { mintEvaluable, snapshotEvaluable } from "$lib/helpers";
  import { fetchToken, type FetchTokenResult } from "@wagmi/core";

  const dispatch = createEventDispatcher();

  export let move: ActionHash;
  export let open: boolean;
  $: _move = hexToBigInt(keccak256(move));

  const price = parseEther("1");

  let allowance: bigint, balance: bigint;

  let token: FetchTokenResult;

  onMount(async () => {
    token = await fetchToken({ address: paymentTokenAddress });
  });

  const getAllowanceAndBalance = () => {
    $paymentToken
      .read({
        functionName: "allowance",
        args: [$account?.address, addresses.instance as Address],
      })
      .then((r) => (allowance = r));

    $paymentToken
      .read({
        functionName: "balanceOf",
        args: [$account?.address],
      })
      .then((r) => (balance = r));
  };

  $: if ($account?.isConnected) getAllowanceAndBalance();

  $: ({ write, status, error } = $nftContract.write({
    functionName: "flow",
    args: [snapshotEvaluable, [_move], []],
    onSuccess: () => {
      dispatch("snapshotMinted");
    },
  }));

  $: ({
    write: allowanceWrite,
    status: allowanceStatus,
    error: allowanceError,
  } = $paymentToken.write({
    functionName: "approve",
    args: [addresses.instance as Address, price],
    onSuccess: () => {
      getAllowanceAndBalance();
    },
  }));

  const snapshotMove = async () => {
    await write();
  };

  $: balanceOk = balance >= price;
  $: allowanceOk = allowance >= price;

  $: ready = balance && allowance && token;

  $: console.log($status);
</script>

<div class="flex flex-col justify-center gap-y-4">
  <Heading tag="h4">Create snapshot</Heading>
  {#if ready}
    {#if !balanceOk}
      <p>You don't have enough {token?.name} to mint this move</p>
    {:else if !allowanceOk}
      <p>
        Before you can mint, you first need to approve the NFT contract to spend
        your {token?.name}
      </p>
      <Button on:click={allowanceWrite}>Approve spend</Button>
    {:else if balanceOk && allowanceOk}
      {#if $status !== "success"}
        <p>
          Creating a snapshot costs {formatUnits(price, token.decimals)}
          {token.symbol}.
        </p>
        <p>
          One you have created your snapshot, other players (and the public)
          will be able to mint your snapshot.
        </p>
        <p>
          The {token.symbol} collected will be sent to the game pool to be redistributed
          to players at the end of the game
        </p>
        <div class="flex gap-x-2">
          <Button
            class="border-2 border-black"
            color="none"
            on:click={() => {
              open = false;
            }}>Cancel</Button
          >
          <Button
            class="bg-fractalorange border-2 border-black"
            disabled={$status === "loading"}
            on:click={snapshotMove}
          >
            {#if $status === "idle" || $status === "error"}
              Mint snapshot
            {:else if $status === "loading"}
              <Spinner size="4" class="mr-2" /> Minting snapshot
            {/if}
          </Button>
        </div>
        {#if $error}
          <p class="text-red-300">{$error?.shortMessage || $error}</p>
        {/if}
      {:else if $status === "success"}
        <p>Snapshot minted!</p>
        <Button
          class="bg-fractalorange border-2 border-black"
          on:click={() => {
            open = false;
          }}>Close</Button
        >
      {/if}
    {/if}

    {#if $allowanceError}
      <p>{$allowanceError}</p>
    {/if}
  {:else}
    <Spinner />
  {/if}
</div>
