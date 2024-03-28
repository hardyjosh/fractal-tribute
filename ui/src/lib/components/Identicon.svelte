<script lang="ts">
  import { HOT_ADDRESS, HOT_TOKEN_THRESHOLD } from "$lib/constants";
  import { happ } from "$lib/stores";
  import type { AgentPubKey } from "@holochain/client";
  import { fetchBalance } from "@wagmi/core";
  import { minidenticon } from "minidenticons";
  import { onMount } from "svelte";
  import { bytesToHex } from "viem";
  import holoLogo from "$lib/assets/holo-logo.svg";

  export let agentHash: AgentPubKey;
  export let size = 40;

  let svgURI: string;
  let hotBalance: bigint;

  onMount(async () => {
    svgURI =
      "data:image/svg+xml;utf8," +
      encodeURIComponent(minidenticon(bytesToHex(agentHash), 100, 60));
    try {
      const agentAccount = await $happ.getAgentEvmAddress(agentHash);
      const agentHotBalance = await fetchBalance({
        chainId: 1,
        address: agentAccount,
        token: HOT_ADDRESS,
      });
      hotBalance = agentHotBalance.value;
    } catch {
      hotBalance = 0n;
    }
  });

  $: hotHolder = hotBalance > HOT_TOKEN_THRESHOLD;
</script>

{#if svgURI}
  <div
    class={`rounded-full aspect-square bg-gray-200 flex flex-col items-center justify-center relative`}
    style={`width: ${size}px; height: ${size}px;`}
    class:hotHolder
  >
    {#if hotHolder}
      <span
        class="absolute rounded-full w-6 h-6 -top-2 -right-2 bg-gray-400 flex items-center justify-center bg-teal-400"
      >
        <img class="w-4" src={holoLogo} />
      </span>
    {/if}
    <img class="w-4/5" src={svgURI} alt="agent identicon" />
  </div>
{/if}

<style lang="postcss">
  .hotHolder {
    @apply shadow-teal-500 shadow-md;
  }
</style>
