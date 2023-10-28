<script lang="ts">
  import { signParticipations } from "$lib/helpers";
  import { happ, web3modal } from "$lib/stores";
  import type { ParticipationProof } from "$lib/types";
  import {
    Button,
    Heading,
    Input,
    Modal,
    Table,
    TableBodyCell,
    TableBodyRow,
  } from "flowbite-svelte";
  import { onMount, onDestroy } from "svelte";
  import { account } from "svelte-wagmi-stores";
  import { getAddress, isHex } from "viem";
  import { privateKeyToAccount } from "viem/accounts";

  let participations: ParticipationProof;
  let signedParticipations: ParticipationProof;
  let pKey: string;

  let error: string = "";
  let submitted: boolean = false;

  const getParticipations = async () => {
    participations = await $happ.buildAgentParticipation();
  };

  function clearLocalStorageAndReload() {
    localStorage.clear();
    location.reload();
  }

  const handlePkeyInput = () => {
    error = "";
    if (!isHex(pKey)) {
      error = "not hex";
      return;
    }
    try {
      const acct = privateKeyToAccount(pKey);
      if (
        getAddress(acct.address) !==
        getAddress($happ.dnaProperties.gameMasterEvmKey)
      ) {
        error = "private key isn't for game steward";
      }
    } catch (e) {
      error = e;
    }
  };

  const handleSignParticipations = async () => {
    error = "";
    try {
      const participations = await $happ.buildAgentParticipation();
      signedParticipations = await signParticipations(participations, pKey);
    } catch (e) {
      error = e;
      return;
    }
  };

  const handleSubmitSignedParticipations = async () => {
    error = "";
    try {
      const record = await $happ.createParticipationProof(signedParticipations);
      if (record.entry) {
        console.log(record.entry);
        submitted = true;
      }
    } catch (e) {
      error = e;
      return;
    }
  };

  // modal
  let open: boolean;

  const handleKeydown = (event: KeyboardEvent) => {
    if (event.metaKey && event.shiftKey && event.code === "Backslash") {
      open = true;
    }
  };

  onMount(() => {
    window.addEventListener("keydown", handleKeydown);
  });

  onDestroy(() => {
    window.removeEventListener("keydown", handleKeydown);
  });
</script>

<Modal bind:open>
  <Heading tag="h3">Admin Modal</Heading>
  {#if participations}
    <Table shadow>
      <TableBodyRow>
        <TableBodyCell>Number of agents</TableBodyCell>
        <TableBodyCell
          >{participations.agent_participations.length}</TableBodyCell
        >
      </TableBodyRow>
      <TableBodyRow>
        <TableBodyCell>Total pixels changed</TableBodyCell>
        <TableBodyCell>{participations.total_pixels_changed}</TableBodyCell>
      </TableBodyRow>
    </Table>
  {/if}
  <div class="flex items-end w-full">
    <button class="text-sm underline" on:click={getParticipations}
      >refresh</button
    >
  </div>
  <Input
    on:input={handlePkeyInput}
    bind:value={pKey}
    placeholder="private key"
  />
  {#if $account?.isConnected}
    <Button
      disabled={error !== "" || !participations || !pKey}
      on:click={handleSignParticipations}>Sign Participations</Button
    >
    <Button
      on:click={handleSubmitSignedParticipations}
      disabled={!signedParticipations}>Submit Signed Participations</Button
    >{:else}
    <Button
      on:click={() => {
        $web3modal.openModal();
      }}>Connect Wallet</Button
    >
  {/if}
  <span class="text-red-500">{error}</span>
  {#if submitted}
    <span class="text-green-500">Submitted!</span>
  {/if}
  <div class="border-t border-gray-200 my-8" />
  <Button on:click={clearLocalStorageAndReload}>Clear Local Storage</Button>
</Modal>
