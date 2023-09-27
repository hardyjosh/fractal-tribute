<script lang="ts">
  import { fade } from "svelte/transition";
  import addresses from "$lib/addresses.json";
  import { account, network } from "svelte-wagmi-stores";
  import { nativeTokenFlowCaller, web3modal } from "$lib/stores";
  import { Alert, Button, Heading, Spinner } from "flowbite-svelte";
  import { parseEther, formatEther, type Hex, type Address } from "viem";
  import { mintEvaluable } from "$lib/helpers";
  import { onMount } from "svelte";
  import { fetchBalance } from "@wagmi/core";
  import { happ } from "$lib/stores";

  export let tokenId: bigint;
  export let open: boolean;

  let balance: bigint;

  $: if ($account?.isConnected)
    fetchBalance({ address: $account.address }).then(
      (r) => (balance = r.value)
    );

  const price = parseEther("0.001");

  let key: Address;
  $: mismatchingKey = $account?.address && key && $account?.address !== key;
  $: wrongNetwork =
    $account?.address && $happ.dnaProperties.chainId !== $network?.chain?.id;

  onMount(async () => {
    key = await $happ.getEvmAddress();
  });

  let hash: Hex;

  $: ({ write, status, error } = $nativeTokenFlowCaller.write({
    functionName: "flow",
    args: [
      addresses.instance,
      mintEvaluable,
      [tokenId, 1n, $account.address],
      [],
    ],
    value: price,
    onSuccess: ({ hash: _hash }) => {
      hash = _hash;
    },
  }));

  const mintMove = async () => {
    await write();
  };

  $: notEnoughBalance = $account?.isConnected && balance < price;

  $: ready = balance !== undefined;
</script>

<div class="flex flex-col justify-center gap-y-4">
  {#if $status == "idle" || $status == "error"}
    <Heading tag="h4">Mint snapshot</Heading>
    <p>
      Minting a snapshot costs {formatEther(price)} MATIC.
    </p>
    <p>
      By minting this snapshot you are helping push this version of the artwork
      up the leaderboard.
    </p>
    <p>
      The MATIC collected will be sent to the game pool to be redistributed to
      players at the end of the game
    </p>
    <div class="flex gap-x-2">
      <Button
        class="border-2 border-black"
        color="none"
        on:click={() => {
          open = false;
        }}>Cancel</Button
      >
      {#if $account?.isConnected}
        <Button
          class="bg-fractalorange border-2 border-black"
          disabled={wrongNetwork || notEnoughBalance || mismatchingKey}
          on:click={mintMove}
        >
          Mint snapshot
        </Button>
      {:else}
        <Button
          class="bg-fractalorange border-2 border-black self-start"
          on:click={() => {
            $web3modal.openModal();
          }}>Connect wallet</Button
        >
      {/if}
    </div>
    {#if wrongNetwork}
      <Alert>
        <p>
          You're connected to the wrong network. Please switch to the{" "}
          {$happ.dnaProperties.chainId === 137 ? "Polygon" : "Mumbai"}{" "}
          network.
        </p>
      </Alert>
    {:else if mismatchingKey}
      <Alert>
        <p>
          You previously bound the Ethereum wallet {key} to your Holochain agent
          key.
        </p>
        <p>
          You'll need switch to this account in your wallet before you can mint.
        </p>
      </Alert>
    {/if}
    {#if notEnoughBalance}
      <Alert>
        <p>
          You don't have enough MATIC to mint this snapshot. Please top up your
          wallet.
        </p>
      </Alert>
    {/if}
    {#if $error}
      <p class="text-red-300">
        {$error?.detail || $error?.shortMessage || $error}
      </p>
    {/if}
  {:else if $status === "loading"}
    <div in:fade class="flex flex-col items-center gap-y-4 my-12">
      <Spinner size="10" class="mr-2" />
      <Heading tag="h4" class="text-center">Minting snapshot</Heading>
      <span>Please check your wallet to confirm</span>
    </div>
  {:else if $status === "success"}
    <div in:fade class="flex flex-col items-center gap-y-4 my-12">
      <Heading class="text-center" tag="h4">Snapshot minted!</Heading>
      <a
        href={`${$network.chain.blockExplorers.default.url}/tx/${hash}`}
        target="_blank"
        class="underline"
      >
        View on explorer</a
      >
      <Button
        class="bg-fractalorange border-2 border-black"
        on:click={() => {
          open = false;
        }}>Done</Button
      >
    </div>
  {/if}
</div>
