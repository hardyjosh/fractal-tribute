<script lang="ts">
  import { signer } from "svelte-ethers-store";
  import { ethers, utils, type BigNumberish, type BytesLike } from "ethers";
  import { type ExpressionConfig, rlc } from "@rainprotocol/rainlang";
  import flowExp from "../../../../rain/nft-flow.rain?raw";
  import canTransferExp from "../../../../rain/can-transfer.rain?raw";
  import FlowERC721Artifact from "../../../../rain/FlowERC721.json";
  import CloneFactoryArtifact from "../../../../rain/CloneFactory.json";
  import config from "../../../../rain/contracts.config.json";

  type EvaluableConfigStruct = {
    deployer: string;
    sources: BytesLike[];
    constants: BigNumberish[];
  };
  type FlowERC721ConfigStruct = {
    name: string;
    symbol: string;
    baseURI: string;
    evaluableConfig: EvaluableConfigStruct;
    flowConfig: EvaluableConfigStruct[];
  };

  const deploy = async () => {
    const configExp = await rlc(canTransferExp, config.deployerOpmeta);
    const flow = await rlc(flowExp, config.deployerOpmeta);

    const flowERC721Config: FlowERC721ConfigStruct = {
      name: "Flow ERC721",
      symbol: "F721",
      baseURI: "https://www.rainprotocol.xyz/nft/",
      evaluableConfig: {
        deployer: config.deployer,
        ...configExp,
      },
      flowConfig: [
        {
          deployer: config.deployer,
          ...flow,
          // sources: [
          //   "0x000d0001000d000300040000000d0005000d0000000d0000000d0000000d0000000d0002000d0002000d0004000d0006",
          // ],
          // constants: [
          //   "0xfea74d0c9bf4a3c28f0dd0674db22a3d7f8bf259c56af19f4ac1e735b156974f",
          //   "0xfe90d819490b07580877ce7c3005704048c62af96c6745886d7e356e0b63924a",
          //   "0x00",
          // ],
        },
      ],
    };

    console.log(flowERC721Config);

    const encodedConfig = ethers.utils.defaultAbiCoder.encode(
      [
        "tuple(string name, string symbol, string baseURI, tuple(address deployer,bytes[] sources,uint256[] constants) evaluableConfig , tuple(address deployer,bytes[] sources,uint256[] constants)[] flowConfig)",
      ],
      [flowERC721Config]
    );

    console.log(ethers.utils.hexlify(encodedConfig));
    const cloneFactory = new ethers.Contract(
      config.cloneFactory,
      CloneFactoryArtifact.abi,
      $signer
    );

    const tx = await cloneFactory.clone(config.flowERC721, encodedConfig);
    const receipt = await tx.wait();
    console.log(receipt);
  };
</script>

<button on:click={deploy}>Deploy</button>
