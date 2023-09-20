import { bytesToHex, getAddress, hexToBytes } from "viem";
import addresses from "$lib/addresses.json"
import type { NFT } from "$lib/types";

export const mintEvaluable = [getAddress(addresses.interpreter), getAddress(addresses.store), getAddress(addresses.mint)]
export const snapshotEvaluable = [getAddress(addresses.interpreter), getAddress(addresses.store), getAddress(addresses.snapshot)]
export const claimEvaluable = [getAddress(addresses.interpreter), getAddress(addresses.store), getAddress(addresses.claim)]

