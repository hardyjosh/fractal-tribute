<script lang="ts">
import { createEventDispatcher, onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import { decode } from '@msgpack/msgpack';
import type { Record, ActionHash, AppAgentClient, EntryHash, AgentPubKey, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { Payload } from './types';
import '@material/mwc-circular-progress';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-snackbar';
import '@material/mwc-icon-button';

const dispatch = createEventDispatcher();

export let payloadHash: ActionHash;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let loading = true;
let error: any = undefined;

let record: Record | undefined;
let payload: Payload | undefined;


  
$:  error, loading, record, payload;

onMount(async () => {
  if (payloadHash === undefined) {
    throw new Error(`The payloadHash input is required for the PayloadDetail element`);
  }
  await fetchPayload();
});

async function fetchPayload() {
  loading = true;
  error = undefined;
  record = undefined;
  payload = undefined;
  
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: 'nft_payload',
      zome_name: 'nft_payload',
      fn_name: 'get_payload',
      payload: payloadHash,
    });
    if (record) {
      payload = decode((record.entry as any).Present.entry) as Payload;
    }
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
<span>Error fetching the payload: {error.data.data}</span>
{:else}

<div style="display: flex; flex-direction: column">
  <div style="display: flex; flex-direction: row">
    <span style="flex: 1"></span>
  </div>

  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="margin-right: 4px"><strong>Name:</strong></span>
    <span style="white-space: pre-line">{ payload.name }</span>
  </div>

  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="margin-right: 4px"><strong>Description:</strong></span>
    <span style="white-space: pre-line">{ payload.description }</span>
  </div>

</div>
{/if}

