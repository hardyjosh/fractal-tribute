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
    nftContract,
    paymentToken,
    paymentTokenAddress,
  } from "$lib/stores";
  import type {
    AgentParticipation,
    Board,
    ParticipationProof,
  } from "$lib/types";
  import {
    Button,
    Spinner,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
  } from "flowbite-svelte";
  import { account } from "svelte-wagmi-stores";
  import { bytesToHex, formatUnits, getAddress, type Hex } from "viem";
  import { writable } from "svelte/store";
  import addresses from "$lib/addresses.json";
  import { fetchToken, type FetchTokenResult } from "@wagmi/core";
  import { setRoute } from "$lib/stores/routes";
  import { createCountdownStore } from "$lib/stores/countdown";

  let board: Board;
  let myParticipation: AgentParticipation;

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
    myParticipation = participations.agent_participations.find(
      (p) => bytesToHex(p.agent) == bytesToHex(myPubKey)
    );
  });

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
    fetchToken({ address: paymentTokenAddress }).then((r) => (token = r));
  }

  $: if (poolSize && token) {
    poolsizeFormatted = formatUnits(poolSize, token.decimals);
  }

  $: ready = myParticipation && poolSize && token && poolsizeFormatted;
</script>

{#if poolSize && token}
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
          >Percentage contribution</TableBodyCell
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
{#if !myParticipation}
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
