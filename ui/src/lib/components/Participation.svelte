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
    // console.log(myPubKey);
    // console.log(participations);
    myParticipation = participations.agent_participations.find(
      (p) => bytesToHex(p.agent) == bytesToHex(myPubKey)
    );

    try {
      poolSize = await $paymentToken.read({
        functionName: "balanceOf",
        args: [addresses.instance as Hex],
      });
    } catch (e) {
      console.log(e);
    }

    // console.log(poolSize);

    token = await fetchToken({ address: paymentTokenAddress });
    // console.log(token);

    poolsizeFormatted = formatUnits(poolSize, token.decimals);
    // console.log(poolsizeFormatted);
  });

  $: ready = myParticipation && poolSize && token && poolsizeFormatted;
  $: console.log({ myParticipation, poolSize, token, poolsizeFormatted });
</script>

{#if poolSize && token}
  <Table>
    <TableBody>
      <TableBodyRow>
        <TableBodyCell>Your pixels changed</TableBodyCell>
        <TableBodyCell>
          {#if myParticipation}
            {myParticipation.pixels_changed}
          {:else}
            0
          {/if}
        </TableBodyCell>
      </TableBodyRow>
      <TableBodyRow>
        <TableBodyCell>Percentage contribution</TableBodyCell>
        <TableBodyCell>
          {#if myParticipation}
            {myParticipation.percentage_of_total_pixels_changed * 100}%
          {:else}
            0%
          {/if}
        </TableBodyCell>
      </TableBodyRow>
      <TableBodyRow>
        <TableBodyCell>Total pool size</TableBodyCell>
        <TableBodyCell>{poolsizeFormatted}</TableBodyCell>
      </TableBodyRow>
      <TableBodyRow>
        <TableBodyCell>Your current allocation</TableBodyCell>
        <TableBodyCell>
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
{/if}
{#if !myParticipation}
  <div>
    <p>Make a move to participate in this artwork!</p>
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
