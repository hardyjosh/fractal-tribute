<script lang="ts">
  import { signer } from "svelte-ethers-store";
  import { ethers } from "ethers";
  import config from "../../../../rain/contracts.config.json";
  import FlowERC721Artifact from "../../../../rain/FlowERC721.json";

  const flowAddress = "0x25001c6b219b88af3d35fdfbd9a0ae7397f9c6f3";
  const expressionAddress = "0xb4184c372461247fc3511a03f62892dfe359821a";

  let tokenId: string;

  type Evaluable = {
    interpreter: string;
    store: string;
    expression: string;
  };

  const mint = async () => {
    const flowERC721 = new ethers.Contract(
      flowAddress,
      FlowERC721Artifact.abi,
      $signer
    );

    const flowConfig: Evaluable = {
      interpreter: config.interpreter,
      store: config.store,
      expression: expressionAddress,
    };
    const tx = await flowERC721.flow(
      flowConfig,
      [ethers.BigNumber.from(tokenId)],
      []
    );
    const receipt = await tx.wait();
    console.log(receipt);
  };
</script>

<input bind:value={tokenId} type="text" />
<button on:click={mint}>Mint!</button>
