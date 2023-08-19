<script lang="ts">
  import { connected } from "svelte-ethers-store";
  import { connectWallet } from "../lib/connect-wallet/";
  import CreatePayload from "../nft_payload/nft_payload/CreatePayload.svelte";
  import CreateEvmKeyBinding from "../nft_payload/nft_payload/CreateEvmKeyBinding.svelte";
  import { getContext, onMount } from "svelte";
  import { getAgentEvmKey } from "../lib/evm-binding/get-evm-key";
  import type { AppAgentClient } from "@holochain/client";
  import { clientContext } from "../contexts";
  import ConnectWallet from "../lib/connect-wallet/ConnectWallet.svelte";

  let evmKey, error;
  let client: AppAgentClient = (getContext(clientContext) as any).getClient();

  const refreshAgentEvmKey = async () => {
    ({ evmKey, error } = await getAgentEvmKey(client));
  };

  onMount(async () => {
    await refreshAgentEvmKey();
  });
</script>

{#if !$connected}
  <ConnectWallet />
{:else if $connected && !evmKey}
  <CreateEvmKeyBinding on:evm-key-binding-created={refreshAgentEvmKey} />
{:else}
  <CreatePayload />
{/if}
