<script lang="ts">
  import { signer } from "svelte-ethers-store";
  import { ethers } from "ethers";
  import config from "../../../../rain/contracts.config.json";
  import FlowERC721Artifact from "../../../../rain/FlowERC721.json";

  const flowAddress = "0x5154d78f89fd638cd2888427cebf44e90e9df62a";
  const expressionAddress = "0x3543e079b2d49e9f8dd9aacaaff6c40433f04ce3";

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
    const tx = await flowERC721.flow(flowConfig, [], []);
    const receipt = await tx.wait();
    console.log(receipt);
  };
</script>

<button on:click={mint}>Mint!</button>
