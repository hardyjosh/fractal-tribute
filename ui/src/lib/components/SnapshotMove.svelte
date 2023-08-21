<script lang="ts">
  import addresses from "$lib/addresses.json";
  import { account } from "svelte-wagmi-stores";
  import { paymentToken } from "./../stores/token.ts";
  import { type ActionHash } from "@holochain/client";
  import { nftContract } from "$lib/stores";
  import { Button } from "flowbite-svelte";
  import { hexToBigInt, keccak256, type Address, parseEther } from "viem";
  import { mintEvaluable, snapshotEvaluable } from "$lib/helpers";

  export let move: ActionHash;
  const _move = hexToBigInt(keccak256(move));

  const price = parseEther("1");

  let allowance: bigint, balance: bigint;

  $: ({ write, status, error } = $nftContract.write({
    functionName: "flow",
    args: [snapshotEvaluable, [_move], []],
  }));

  $: ({
    write: allowanceWrite,
    status: allowanceStatus,
    error: allowanceError,
  } = $paymentToken.write({
    functionName: "approve",
    args: [addresses.instance as Address, price],
  }));

  const snapshotMove = async () => {
    await write();
  };

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

  $: console.log($error);

  $: balanceOk = balance >= price;
  $: allowanceOk = allowance >= price;

  $: console.log({ allowance, balance, price });
</script>

{#if !balanceOk}
  <p>You don't have enough balance to mint this move</p>
{/if}

{#if !allowanceOk}
  <p>You need to approve the contract to spend your tokens</p>
  <Button on:click={allowanceWrite}>Approve</Button>
{/if}

{#if balanceOk && allowanceOk}
  <Button on:click={snapshotMove}>Snapshot my move</Button>
{/if}
