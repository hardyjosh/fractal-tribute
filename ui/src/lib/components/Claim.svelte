<script lang="ts">
  import { fade } from "svelte/transition";
  import { claimEvaluable, constructSignedContext } from "$lib/helpers";
  import { happ, nftContract, web3modal } from "$lib/stores";
  import type { AgentParticipation, ParticipationProof } from "$lib/types";
  import { Button, Heading, Modal, Spinner } from "flowbite-svelte";
  import { onMount } from "svelte";
  import { walletClient, network } from "svelte-wagmi-stores";
  import type { Address, Hex } from "viem";
  import { Confetti } from "svelte-confetti";

  let open: boolean = false;
  export let participations: ParticipationProof;
  export let evmKey: Address = null;
  let agentParticipation: AgentParticipation;

  onMount(async () => {
    if (!evmKey) evmKey = await $happ.getEvmAddress();
    agentParticipation = await $happ.getSignedParticipation(evmKey);
  });

  let hash: Hex;

  $: ({ write, status, error } = $nftContract.write({
    functionName: "flow",
    args: [
      claimEvaluable,
      [],
      [
        agentParticipation &&
          constructSignedContext(
            agentParticipation,
            $happ.dnaProperties.gameMasterEvmKey,
            participations.total_pixels_changed
          ),
      ],
    ],
    onSuccess: ({ hash: _hash }) => {
      hash = _hash;
    },
  }));

  $: console.log($error, $status, write);

  const claim = async () => {
    write();
  };
</script>

<Button
  size="xs"
  class="bg-fractalorange border-2 border-black"
  on:click={() => (open = true)}>Claim</Button
>

<Modal bind:open>
  {#if ($status == "idle" || $status == "error") && !hash}
    <div in:fade class="flex flex-col items-center gap-y-4 my-12">
      <Heading tag="h4" class="text-center">Claim</Heading>
      <p>Claim your share of the pool. You can only claim once.</p>
      {#if $walletClient}
        <Button class="bg-fractalorange border-2 border-black" on:click={claim}
          >Claim</Button
        >
      {:else}
        <Button
          class="bg-fractalorange border-2 border-black"
          on:click={() => $web3modal.openModal()}>Connect</Button
        >
      {/if}
      {#if $error}
        {#if $error?.message?.includes("0x160f9f14")}
          <p class="text-red-500">
            Something went wrong. Have you already claimed?
          </p>
        {:else}
          <p class="text-red-500">
            {$error?.shortMessage || "Something went wrong"}
          </p>
        {/if}
      {/if}
    </div>
  {:else if $status === "loading"}
    <div in:fade class="flex flex-col items-center gap-y-4 my-12">
      <Spinner size="10" class="mr-2" />
      <Heading tag="h4" class="text-center">Claiming...</Heading>
      <span>Please check your wallet to confirm</span>
    </div>
  {:else if $status === "success" || hash}
    <div class="fixed inset-0 translate-x-1/2 pointer-events-none">
      <Confetti
        x={[-4, 4]}
        y={[0, 1]}
        fallDistance="1500px"
        amount={500}
        cone={true}
        delay={[0, 700]}
      />
    </div>
    <div in:fade class="flex flex-col items-center gap-y-4 my-12">
      <div class="rounded-full bg-gray-100 p-6 text-4xl">🎉</div>
      <Heading class="text-center" tag="h4">Claim successful!</Heading>
      <p>
        Thank you for playing Fractal Tribute!
        <a
          href={`${$network.chain.blockExplorers.default.url}/tx/${hash}`}
          target="_blank"
          class="underline"
        >
          View on explorer</a
        >
      </p>
      <Button
        class="bg-fractalorange border-2 border-black"
        on:click={() => {
          open = false;
        }}>Done</Button
      >
    </div>
  {/if}
</Modal>
