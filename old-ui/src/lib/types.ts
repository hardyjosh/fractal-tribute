export enum Routes {
  Home = "home",
  MintNFT = "mint-nft",
  ViewNFTs = "view-nfts",
}

export type Color = {
  r: number,
  g: number,
  b: number
};

export type PixelChange = {
  x: number,
  y: number,
  color: Color,
  graphic_option: number
};

export type GameMove = {
  changes: PixelChange[]
};

export type Tile = {
  color?: Color,
  graphic_option?: number
};
