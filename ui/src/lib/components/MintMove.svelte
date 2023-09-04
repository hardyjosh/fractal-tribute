<script lang="ts">
  import addresses from "$lib/addresses.json";
  import { account } from "svelte-wagmi-stores";
  import { type ActionHash } from "@holochain/client";
  import { nftContract, paymentToken } from "$lib/stores";
  import { Button } from "flowbite-svelte";
  import { hexToBigInt, keccak256, type Address, parseEther } from "viem";
  import {
    claimEvaluable,
    mintEvaluable,
    snapshotEvaluable,
  } from "$lib/helpers";

  export let tokenId: bigint;
  export let open: boolean;

  let allowance: bigint, balance: bigint;
  const price = parseEther("1");

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

  // $: console.log({ allowance, balance, price });
</script>

{#if !balanceOk}
  <p>You don't have enough balance to mint this move</p>
{/if}

{#if !allowanceOk}
  <p>You need to approve the contract to spend your tokens</p>
  <Button on:click={allowanceWrite}>Approve</Button>
{/if}

{#if balanceOk && allowanceOk}
  <Button on:click={mintMove}>Mint</Button>
{/if}
