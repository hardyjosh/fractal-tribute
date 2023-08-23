<script lang="ts">
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
  import { happ, nftContract } from "$lib/stores";
  import type { Board } from "$lib/types";
  import { Button } from "flowbite-svelte";
  import { account } from "svelte-wagmi-stores";
  import { bytesToHex, getAddress } from "viem";
  import { writable } from "svelte/store";

  let board: Board;

  const getBoard = async () => {
    board = await $happ.getLatestBoard();
  };

  let error = writable(null),
    write;

  const buildParticipation = async () => {
    const res = await $happ.buildAgentParticipation();
    const signedRes = await signParticipations(res);
    const myParticipation = signedRes.agent_participations.find(
      (p) => getAddress(bytesToHex(p.evm_key)) == $account?.address
    );
    const signedContext = constructSignedContext(myParticipation);
    console.log({ signedContext });
    console.log({ myParticipation });
    ({ write, error } = $nftContract.write({
      functionName: "flow",
      args: [claimEvaluable, [], [constructSignedContext(myParticipation)]],
    }));
    await write();
  };

  $: console.log($error);
</script>

<Button on:click={buildParticipation}>Build participation</Button>
