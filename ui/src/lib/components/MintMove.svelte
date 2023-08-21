<script lang="ts">
  import { type ActionHash } from "@holochain/client";
  import { evaluable, token } from "$lib/stores";
  import { Button } from "flowbite-svelte";
  import { hexToBigInt, keccak256 } from "viem";

  export let move: ActionHash;

  const _move = hexToBigInt(keccak256(move));

  $: ({ write, status, error } = $token.write({
    functionName: "flow",
    args: [evaluable, [_move], []],
  }));

  const mintMove = async () => {
    await write();
  };
</script>

<Button on:click={mintMove}>Mint my move</Button>
