import { ethers } from "ethers";
import config from "../../../../rain/contracts.config.json";
import FlowERC721Artifact from "../../../../rain/FlowERC721.json";

const flowAddress = config.deployedFlow;
const expressionAddress = config.deployedExpression;

type Evaluable = {
    interpreter: string;
    store: string;
    expression: string;
};

export const mint = async (contentHash, signer) => {
    const flowERC721 = new ethers.Contract(
        flowAddress,
        FlowERC721Artifact.abi,
        signer
    );

    const flowConfig: Evaluable = {
        interpreter: config.interpreter,
        store: config.store,
        expression: expressionAddress,
    };
    const tx = await flowERC721.flow(
        flowConfig,
        [ethers.BigNumber.from(contentHash)],
        []
    );
    const receipt = await tx.wait();
    return receipt;
};