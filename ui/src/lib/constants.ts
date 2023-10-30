import { parseEther } from "viem";

export const CHANGES_PER_MOVE = 20;
export const price = parseEther("0.02");
export const ADDITIONAL_MINT_PERIOD = 86400 * 1000 // 1000 * 60 * 60 * 24; // 1 day in milliseconds

export const HOT_ADDRESS = "0x6c6ee5e31d828de241282b9606c8e98ea48526e2"
export const HOT_TOKEN_THRESHOLD = parseEther("1000");
