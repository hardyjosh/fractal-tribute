import { IFlowERC1155V3 } from "$lib/abi/IFlowERC1155V3";
import type { ActionHash } from "@holochain/client";
import { makeContractStore } from "svelte-wagmi-stores";
import { get } from "svelte/store";
import type { Address } from "viem";
import { getAddress } from "viem";

const addresses = {
    "expression": "0x0046eAC07579c27185CFBB044721f85174382170",
    "instance": "0x8C7aB420A7a0fd520193302EF7937d3d0b4BAbc4",
    "interpreter": "0x69c0783A613d33E19Ad91aeD51A616a4b27aE3a2",
    "store": "0xeCe6950c86BbD6F7a1A4CEEf6254B7C922dDd4F0"
}
export const evaluable = [getAddress(addresses.interpreter), getAddress(addresses.store), getAddress(addresses.expression)]

export const token = makeContractStore(IFlowERC1155V3, addresses.instance as Address)

// export const mintToken = (hash: ActionHash) => {
//     console.log(hash)
//     const _token = get(token)
//     return _token.write({
//         functionName: "flow",
//         args: [
//             [addresses.interpreter, getAddress(addresses.store), getAddress(addresses.expression)],
//             [hash],
//             []
//         ]
//     })
// }