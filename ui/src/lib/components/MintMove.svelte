<script lang="ts">
  import addresses from "$lib/addresses.json";
  import { account } from "svelte-wagmi-stores";
  import { nativeTokenFlowCaller, web3modal } from "$lib/stores";
  import { Button, Heading, Spinner } from "flowbite-svelte";
  import { parseEther, formatEther } from "viem";
  import { mintEvaluable } from "$lib/helpers";
  import { onMount } from "svelte";
  import { fetchBalance } from "@wagmi/core";

  export let tokenId: bigint;
  export let open: boolean;

  let balance: bigint;

  $: if ($account?.isConnected)
    fetchBalance({ address: $account.address }).then(
      (r) => (balance = r.value)
    );

  const price = parseEther("0.001");

  onMount(async () => {});

  $: ({ write, status, error } = $nativeTokenFlowCaller.write({
    functionName: "flow",
    args: [
      addresses.instance,
      mintEvaluable,
      [tokenId, 1n, $account.address],
      [],
    ],
    value: price,
  }));

  const mintMove = async () => {
    await write();
  };

  $: balanceOk = balance >= price;

  $: ready = balance !== undefined;
</script>

<div class="flex flex-col justify-center gap-y-4">
  <Heading tag="h4">Mint snapshot</Heading>
  {#if !$account.isConnected}
    <p>You need to connect your wallet to collect a snapshot</p>
    <Button
      class="bg-fractalorange border-2 border-black self-start"
      on:click={() => {
        $web3modal.openModal();
      }}>Connect wallet</Button
    >
  {:else if $account.isConnected && !ready}
    <Spinner />
  {:else if $account.isConnected && ready}
    {#if balanceOk}
      {#if $status !== "success"}
        <p>
          Minting a snapshot costs {formatEther(price)} MATIC.
        </p>
        <p>
          By minting this snapshot you are helping push this version of the
          artwork up the leaderboard.
        </p>
        <p>
          The MATIC collected will be sent to the game pool to be redistributed
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
  {/if}
</div>
