<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { EntryHash, Record, AgentPubKey, ActionHash, AppAgentClient, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../contexts';
import PayloadDetail from './PayloadDetail.svelte';
import type { NftPayloadSignal } from './types';


let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let hashes: Array<ActionHash> | undefined;
let loading = true;
let error: any = undefined;

$: hashes, loading, error;

onMount(async () => {

  await fetchPayloads();
  client.on('signal', signal => {
    if (signal.zome_name !== 'nft_payload') return;
    const payload = signal.payload as NftPayloadSignal;
    if (payload.type !== 'EntryCreated') return;
    if (payload.app_entry.type !== 'Payload') return;
    hashes = [...hashes, payload.action.hashed.hash];
  });
});

async function fetchPayloads() {
  try {
    const records = await client.callZome({
      cap_secret: null,
      role_name: 'nft_payload',
      zome_name: 'nft_payload',
      fn_name: 'get_all_payloads',
      payload: null,
    });
    hashes = records.map(r => r.signed_action.hashed.hash);
  } catch (e) {
    error = e;
  }
  loading = false;
}

</script>

{#if loading}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the payloads: {error.data.data}.</span>
{:else if hashes.length === 0}
<span>No payloads found.</span>
{:else}
<div style="display: flex; flex-direction: column">
  {#each hashes as hash}
    <div style="margin-bottom: 8px;">
      <PayloadDetail payloadHash={hash}  on:payload-deleted={() => fetchPayloads()}></PayloadDetail>
    </div>
  {/each}
</div>
{/if}

