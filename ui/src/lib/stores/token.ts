import { IFlowERC1155V3 } from "$lib/abi/IFlowERC1155V3";
import { makeContractStore } from "svelte-wagmi-stores";
import type { Address } from "viem";
import { getAddress } from "viem";
import addresses from "$lib/addresses.json";

export const token = makeContractStore(IFlowERC1155V3, addresses.instance as Address)