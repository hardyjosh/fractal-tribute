<script lang="ts">
  import { onMount } from "svelte";
  import { get } from "svelte/store";
  import AllMyMoves from "$lib/components/AllMyMoves.svelte";
  import AllNfts from "$lib/components/AllNfts.svelte";
  import BoardComp from "$lib/components/Board.svelte";
  import CreateEvmKeyBinding from "$lib/components/CreateEvmKeyBinding.svelte";
  import { claimEvaluable } from "$lib/helpers";
  import { generateRandomGameMove } from "$lib/helpers/game-move";
  import {
    signParticipations,
    constructSignedContext,
  } from "$lib/helpers/participation";
  import {
    happ,
    paymentToken,
    paymentTokenAddress,
    web3modal,
  } from "$lib/stores";
  import type { AgentParticipation, Board } from "$lib/types";
  import {
    Button,
    Spinner,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
  } from "flowbite-svelte";
  import { account, walletClient } from "svelte-wagmi-stores";
  import {
    bytesToHex,
    formatUnits,
    getAddress,
    type Address,
    type Hex,
  } from "viem";
  import { writable } from "svelte/store";
  import addresses from "$lib/addresses.json";
  import { fetchToken, type FetchTokenResult } from "@wagmi/core";
  import { setRoute } from "$lib/stores/routes";
  import { createCountdownStore } from "$lib/stores/countdown";

  let board: Board;
  let myParticipation: AgentParticipation;
  let evmKey: Address;

  let bindingActive: boolean;

  let error = writable(null),
    write;

  // const claim = async () => {
  //   const signedRes = await signParticipations(res);
  //   const myParticipation = signedRes.agent_participations.find(
  //     (p) => getAddress(bytesToHex(p.evm_key)) == $account?.address
  //   );
  //   const signedContext = constructSignedContext(myParticipation);
  //   // console.log({ signedContext });
  //   // console.log({ myParticipation });
  //   ({ write, error } = $nftContract.write({
  //     functionName: "flow",
  //     args: [claimEvaluable, [], [constructSignedContext(myParticipation)]],
  //   }));
  //   await write();
  // };

  let poolSize: bigint, token: FetchTokenResult, poolsizeFormatted: string;

  onMount(async () => {
    const participations = await $happ.buildAgentParticipation();
    const myPubKey = await $happ.myPubKey();
    evmKey = await $happ.getEvmAddress();
    console.log(evmKey);
    myParticipation = participations.agent_participations.find(
      (p) => bytesToHex(p.agent) == bytesToHex(myPubKey)
    );
  });

  const handleEvmBindingCreated = async () => {
    evmKey = await $happ.getEvmAddress();
    bindingActive = false;
  };

  $: if ($account?.isConnected) {
    try {
      $paymentToken
        .read({
          functionName: "balanceOf",
          args: [addresses.instance as Hex],
        })
        .then((r) => (poolSize = r));
    } catch (e) {
      console.log(e);
    }
    fetchToken({ address: $paymentTokenAddress }).then((r) => (token = r));
  }

  $: if (poolSize && token) {
    poolsizeFormatted = formatUnits(poolSize, token.decimals);
  }

  $: ready = myParticipation && poolSize && token && poolsizeFormatted;
</script>

{#if !evmKey && bindingActive}
  <div class="w-full">
    <CreateEvmKeyBinding on:evmKeyBindingCreated={handleEvmBindingCreated} />
  </div>
{:else if !evmKey}
  <div class="mt-8 px-8 text-center flex flex-col gap-y-4">
    <div class="text-xl">
      You haven't bound your Ethereum wallet to your Holochain agent key yet.
    </div>
    <div class="text-xl">
      Do it before the game ends to claim your share of the pool after minting
      closes.
    </div>
    <Button
      class="bg-fractalorange border-2 border-black"
      on:click={() => {
        bindingActive = true;
      }}>Bind now</Button
    >
  </div>
{:else if !$walletClient}
  <div class="mt-8 flex flex-col items-center gap-y-6">
    <p class="text-center">Connect your wallet to get your stats</p>
    <Button
      class="bg-fractalorange border-2 border-black"
      on:click={() => {
        $web3modal.openModal();
      }}>Connect wallet</Button
    >
  </div>
{:else if poolSize && token}
  <Table divClass="w-full bg-none" noborder>
    <TableBody>
      <TableBodyRow class="bg-none!">
        <TableBodyCell class="text-xl font-light"
          >Your pixels changed</TableBodyCell
        >
        <TableBodyCell class="text-right font-bold text-xl">
          {#if myParticipation}
            {myParticipation.pixels_changed}
          {:else}
            0
          {/if}
        </TableBodyCell>
      </TableBodyRow>
      <TableBodyRow class="bg-none!">
        <TableBodyCell class="text-xl font-light"
          >Percentage allocation</TableBodyCell
        >
        <TableBodyCell class="text-right font-bold text-xl">
          {#if myParticipation}
            {myParticipation.percentage_of_total_pixels_changed * 100}%
          {:else}
            0%
          {/if}
        </TableBodyCell>
      </TableBodyRow>
      <TableBodyRow class="bg-none!">
        <TableBodyCell class="text-xl font-light">Total pool size</TableBodyCell
        >
        <TableBodyCell class="text-right font-bold text-xl"
          >{poolsizeFormatted}</TableBodyCell
        >
      </TableBodyRow>
      <TableBodyRow class="bg-none!">
        <TableBodyCell class="text-xl font-light"
          >Your current allocation</TableBodyCell
        >
        <TableBodyCell class="text-right font-bold text-xl">
          {#if myParticipation}
            {myParticipation.percentage_of_total_pixels_changed *
              Number(poolsizeFormatted)}
          {:else}
            0
          {/if}
        </TableBodyCell>
      </TableBodyRow>
    </TableBody>
  </Table>
{:else}
  <div>
    <Spinner class="mr-4" /> Calculating your participation...
  </div>
{/if}
{#if !myParticipation && evmKey}
  <div
    class="flex flex-col items-center gap-y-4 border-gray-200 border-t w-full pt-8 mt-8"
  >
    <p class="text-xl">Make a move to participate in this artwork!</p>
    <Button
      on:click={() => {
        setRoute("Play");
      }}
      class="bg-fractalorange border-2 border-black">Make move</Button
    >
  </div>
{/if}
<!-- <Button on:click={buildParticipation}>Get participation</Button> -->
<!-- {#if res}
  <pre>{JSON.stringify(res, null, 2)}</pre>
{/if} -->
<!-- <Button on:click={claim}>Claim</Button> -->
