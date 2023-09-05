<script lang="ts">
  import addresses from "$lib/addresses.json";
  import { account } from "svelte-wagmi-stores";
  import {
    nftContract,
    paymentToken,
    paymentTokenAddress,
    web3modal,
  } from "$lib/stores";
  import { Button, Heading, Spinner } from "flowbite-svelte";
  import { type Address, parseEther, formatUnits } from "viem";
  import { mintEvaluable } from "$lib/helpers";
  import { fetchToken, type FetchTokenResult } from "@wagmi/core";
  import { onMount } from "svelte";

  export let tokenId: bigint;
  export let open: boolean;

  let allowance: bigint, balance: bigint;
  let token: FetchTokenResult;

  const price = parseEther("1");

  onMount(async () => {
    token = await fetchToken({ address: paymentTokenAddress });
  });

  $: ({ write, status, error } = $nftContract.write({
    functionName: "flow",
    args: [mintEvaluable, [tokenId, 1n], []],
  }));

  const mintMove = async () => {
    await write();
  };

  $: ({
    write: allowanceWrite,
    status: allowanceStatus,
    error: allowanceError,
  } = $paymentToken.write({
    functionName: "approve",
    args: [addresses.instance as Address, price],
  }));

  $: $paymentToken
    .read({
      functionName: "allowance",
      args: [$account?.address, addresses.instance as Address],
    })
    .then((r) => (allowance = r));

  $: $paymentToken
    .read({
      functionName: "balanceOf",
      args: [$account?.address],
    })
    .then((r) => (balance = r));

  // $: console.log($error);

  $: balanceOk = balance >= price;
  $: allowanceOk = allowance >= price;

  $: ready =
    balance !== undefined && allowance !== undefined && token !== undefined;

  // $: console.log({ allowance, balance, price });
</script>

<div class="flex flex-col justify-center gap-y-4">
  <Heading tag="h4">Mint snapshot</Heading>
  {#if !$account.isConnected}
    <p>You need to connect your wallet to mint a snapshot</p>
    <Button
      class="bg-fractalorange border-2 border-black self-start"
      on:click={() => {
        $web3modal.openModal();
      }}>Connect wallet</Button
    >
  {:else if $account.isConnected && !ready}
    <Spinner />
  {:else if $account.isConnected && ready}
    {#if !balanceOk}
      <p>You don't have enough {token?.name} to mint this move</p>
    {:else if !allowanceOk}
      <p>
        Before you can mint, you first need to approve the NFT contract to spend
        your {token?.name}
      </p>
      <Button
        class="bg-fractalorange border-2 border-black self-start  "
        disabled={$allowanceStatus === "loading"}
        on:click={allowanceWrite}
      >
        {#if $allowanceStatus === "loading"}
          <Spinner size="4" class="mr-2" /> Approving
        {:else}
          Approve spend
        {/if}
      </Button>
    {:else if balanceOk && allowanceOk}
      {#if $status !== "success"}
        <p>
          Minting a snapshot costs {formatUnits(price, token.decimals)}
          {token.symbol}.
        </p>
        <p>
          By minting this snapshot you are helping push this version of the
          artwork up the leaderboard.
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
            on:click={mintMove}
          >
            {#if $status === "loading"}
              <Spinner size="4" class="mr-2" /> Minting snapshot
            {:else}
              Mint snapshot
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
  {/if}
</div>
