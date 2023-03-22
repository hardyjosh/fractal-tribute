<script lang="ts">
import { createEventDispatcher, onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import { decode } from '@msgpack/msgpack';
import type { Record, ActionHash, AppAgentClient, EntryHash, AgentPubKey, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { EvmKeyBinding } from './types';
import '@material/mwc-circular-progress';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-snackbar';
import '@material/mwc-icon-button';

const dispatch = createEventDispatcher();

export let evmKeyBindingHash: ActionHash;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let loading = true;
let error: any = undefined;

let record: Record | undefined;
let evmKeyBinding: EvmKeyBinding | undefined;


  
$:  error, loading, record, evmKeyBinding;

onMount(async () => {
  if (evmKeyBindingHash === undefined) {
    throw new Error(`The evmKeyBindingHash input is required for the EvmKeyBindingDetail element`);
  }
  await fetchEvmKeyBinding();
});

async function fetchEvmKeyBinding() {
  loading = true;
  error = undefined;
  record = undefined;
  evmKeyBinding = undefined;
  
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: 'nft_payload',
      zome_name: 'nft_payload',
      fn_name: 'get_evm_key_binding',
      payload: evmKeyBindingHash,
    });
    if (record) {
      evmKeyBinding = decode((record.entry as any).Present.entry) as EvmKeyBinding;
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
<span>Error fetching the evm key binding: {error.data.data}</span>
{:else}

<div style="display: flex; flex-direction: column">
  <div style="display: flex; flex-direction: row">
    <span style="flex: 1"></span>
  </div>

  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="margin-right: 4px"><strong>Evm Key:</strong></span>
    <span style="white-space: pre-line">{ evmKeyBinding.evm_key }</span>
  </div>

</div>
{/if}

