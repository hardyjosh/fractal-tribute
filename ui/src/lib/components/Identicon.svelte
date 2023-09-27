<script lang="ts">
  import type { AgentPubKey } from "@holochain/client";
  import { minidenticon } from "minidenticons";
  import { onMount } from "svelte";
  import { bytesToHex } from "viem";

  export let agentHash: AgentPubKey;
  export let size = 40;
  let svgURI: string;
  onMount(() => {
    svgURI =
      "data:image/svg+xml;utf8," +
      encodeURIComponent(minidenticon(bytesToHex(agentHash), 100, 60));
  });
</script>

{#if svgURI}
  <div
    class={`rounded-full aspect-square bg-gray-200 flex flex-col items-center justify-center`}
    style={`width: ${size}px; height: ${size}px;`}
  >
    <img class="w-4/5" src={svgURI} alt="agent identicon" />
  </div>
{/if}
