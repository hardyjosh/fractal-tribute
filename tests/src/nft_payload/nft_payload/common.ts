import { CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeActionHash, fakeAgentPubKey, fakeEntryHash, fakeDnaHash } from '@holochain/client';



export async function samplePayload(cell: CallableCell, partialPayload = {}) {
    return {
        ...{
          creator: cell.cell_id[1],
	  name: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        },
        ...partialPayload
    };
}

export async function createPayload(cell: CallableCell, payload = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "nft_payload",
      fn_name: "create_payload",
      payload: payload || await samplePayload(cell),
    });
}



export async function sampleEvmKeyBinding(cell: CallableCell, partialEvmKeyBinding = {}) {
    return {
        ...{
	  evm_key: 10,
          creator: cell.cell_id[1],
        },
        ...partialEvmKeyBinding
    };
}

export async function createEvmKeyBinding(cell: CallableCell, evmKeyBinding = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "nft_payload",
      fn_name: "create_evm_key_binding",
      payload: evmKeyBinding || await sampleEvmKeyBinding(cell),
    });
}

