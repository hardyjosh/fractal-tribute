<script lang="ts">
  import {
    encodeHashToBase64,
    type ActionHash,
    decodeHashFromBase64,
  } from "@holochain/client";
  import { evaluable, token } from "$lib/stores";
  import { Button } from "flowbite-svelte";
  import {
    bytesToBigint,
    hashMessage,
    hexToBigInt,
    concat,
    numberToHex,
    bytesToHex,
    hexToBytes,
  } from "viem";

  export let move: ActionHash;

  const _move = hexToBigInt(hashMessage({ raw: move }));

  $: ({ write, status, error } = $token.write({
    functionName: "flow",
    args: [evaluable, [_move], []],
  }));

  //   $: console.log({ $status, $error, write });

  const mintMove = async () => {
    console.log("minting move");
    console.log(move);
    await write();
  };

  const linkBase = concat([
    // "0x842f24",
    numberToHex(
      BigInt(
        "64827889298651374156721083168816601664438786633855154501320660309338545698782"
      )
    ),
    bytesToHex(Uint8Array.from([0, 0, 0, 0])),
  ]);

  console.log(
    "decoded",
    decodeHashFromBase64(
      "uhC8kBsvGwL93EPtMy9fsp6ZjaMf6XTeW-D2WBW0m2INJKikAAAAA"
    )
  );
  console.log("link base", hexToBytes(linkBase));
  console.log("length", move.length);
</script>

<Button on:click={mintMove}>Mint my move</Button>

<!-- {
    "type": "CreateLink",
    "author": "uhCAk_gB5wpZRO50Oj1yK4kI7akDaw0uXfObYvGRxT7QaGtnRImky",
    "timestamp": 1692567290857436,
    "action_seq": 9,
    "prev_action": "uhCkkfjzeTruIHRpdPkMaqGJcXqnKInc9dPC_Gkhnpve6UTN1LLx7",
    "base_address": "uhC8kBsvGwL93EPtMy9fsp6ZjaMf6XTeW-D2WBW0m2INJKikAAAAA",
    "target_address": "uhCkkfjzeTruIHRpdPkMaqGJcXqnKInc9dPC_Gkhnpve6UTN1 -->
