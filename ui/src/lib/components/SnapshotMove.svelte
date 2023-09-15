<script lang="ts">
  import { onMount, createEventDispatcher } from "svelte";
  import addresses from "$lib/addresses.json";
  import { account, walletClient } from "svelte-wagmi-stores";
  import { type ActionHash } from "@holochain/client";
  import {
    happ,
    nftContract,
    paymentToken,
    paymentTokenAddress,
    web3modal,
  } from "$lib/stores";
  import { Button, Spinner, Heading } from "flowbite-svelte";
  import {
    hexToBigInt,
    keccak256,
    type Address,
    parseEther,
    formatUnits,
  } from "viem";
  import { snapshotEvaluable } from "$lib/helpers";
  import {
    fetchToken,
    waitForTransaction,
    type FetchTokenResult,
  } from "@wagmi/core";
  import CreateEvmKeyBinding from "$lib/components/CreateEvmKeyBinding.svelte";

  const dispatch = createEventDispatcher();

  export let move: ActionHash;
  export let open: boolean;

  $: _move = hexToBigInt(keccak256(move));

  const price = parseEther("0");

  let allowance: bigint, balance: bigint;
  let token: FetchTokenResult;

  let key: Address;
  let checkedKey = false;

  onMount(async () => {
    token = await fetchToken({ address: $paymentTokenAddress });
    await getKey();
    checkedKey = true;
  });

  const getKey = async () => {
    key = await $happ.getEvmAddress();
  };

  const getAllowanceAndBalance = async () => {
    allowance = await $paymentToken.read({
      functionName: "allowance",
      args: [$account?.address, addresses.instance as Address],
    });

    balance = await $paymentToken.read({
      functionName: "balanceOf",
      args: [$account?.address],
    });
  };

  $: if ($walletClient) getAllowanceAndBalance();

  $: ({ write, status, error } = $nftContract.write({
    functionName: "flow",
    args: [snapshotEvaluable, [_move], []],
    onSuccess: ({ hash }) => {
      waitForTransaction({ hash, confirmations: 5 }).then(() => {
        dispatch("snapshotMinted");
      });
    },
  }));

  $: ({
    write: allowanceWrite,
    status: allowanceStatus,
    error: allowanceError,
  } = $paymentToken.write({
    functionName: "approve",
    args: [addresses.instance as Address, price],
    onSuccess: ({ hash }) => {
      waitForTransaction({ hash, confirmations: 5 }).then(() => {
        getAllowanceAndBalance();
      });
    },
  }));

  const snapshotMove = async () => {
    await $happ.createTokenIdForGameMove(move);
    await write();
  };

  $: balanceOk = balance >= price;
  $: allowanceOk = allowance >= price;

  $: ready =
    balance !== undefined && allowance !== undefined && token !== undefined;

  $: console.log($error);
</script>

<div class="flex flex-col justify-center gap-y-4">
  <Heading tag="h4">Create snapshot</Heading>
  {#if !checkedKey}
    <Spinner />
  {:else if !key}
    <p>
      Before you can mint a snapshot you need to bind your Ethereum wallet to
      your Holochain agent key.
    </p>
    {#if !$account.isConnected}
      <Button
        class="bg-fractalorange border-2 border-black self-start"
        on:click={() => {
          $web3modal.openModal();
        }}>Connect wallet</Button
      >
    {:else}
      <CreateEvmKeyBinding on:evmKeyBindingCreated={getKey} />
    {/if}
  {:else if !$account.isConnected}
    <p>You need to connect your wallet to mint a snapshot</p>
    <Button
      class="bg-fractalorange border-2 border-black self-start"
      on:click={() => {
        $web3modal.openModal();
      }}>Connect wallet</Button
    >
  {:else if $account?.isConnected && $account.address !== key}
    <p>
      You previously bound the Ethereum wallet {key} to your Holochain agent key.
    </p>
    <p>
      You'll need switch to this account in your wallet before you can mint.
    </p>
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
          <!-- Creating a snapshot costs {formatUnits(price, token.decimals)}
          {token.symbol}. -->
          Creating a snapshot is free, you'll just need to pay the gas fee.
        </p>
        <p>
          One you have created your snapshot, other players (and the public)
          will be able to mint your snapshot to push it up the leaderboard.
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
            on:click={snapshotMove}
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
