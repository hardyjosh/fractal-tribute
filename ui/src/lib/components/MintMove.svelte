<script lang="ts">
  import { type ActionHash } from "@holochain/client";
  import { token } from "$lib/stores";
  import { Button } from "flowbite-svelte";
  import { hexToBigInt, keccak256 } from "viem";
  import { mintEvaluable } from "$lib/helpers";

  export let move: ActionHash;

  const _move = hexToBigInt(keccak256(move));

  $: ({ write, status, error } = $token.write({
    functionName: "flow",
    args: [mintEvaluable, [_move], []],
  }));

  const mintMove = async () => {
    await write();
  };
</script>

<Button on:click={mintMove}>Mint my move</Button>
