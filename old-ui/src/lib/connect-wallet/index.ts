import Web3Modal from 'web3modal';
import { defaultEvmStores } from 'svelte-ethers-store';
import WalletConnectProvider from '@walletconnect/web3-provider/dist/umd/index.min';

const providerOptions = {
    injected: {
        display: {
            name: 'Metamask',
            description: 'Connect with the provider in your Browser'
        },
        package: null
    },
    walletconnect: {
        package: WalletConnectProvider,
        options: {
            infuraId: '0f270373e0934beda174c537257386b0',
            rpc: {
                80001: 'https://rpc-mumbai.maticvigil.com'
            }
        }
    }
};

export const connectWallet = async () => {
    const web3Modal = new Web3Modal({
        cacheProvider: true,
        providerOptions
    });
    const provider = await web3Modal.connect();
    // ethersProvider = new providers.Web3Provider(provider);
    defaultEvmStores.setProvider(provider);
};

export const disconectWallet = async () => {
    defaultEvmStores.disconnect();
};