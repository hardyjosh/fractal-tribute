<script lang="ts">
    import { Button } from 'flowbite-svelte';
    import { configureChains } from '@wagmi/core'
    import { mainnet, polygon } from '@wagmi/core/chains'
    import { createConfig, account } from 'svelte-wagmi-stores';
    // this example also uses Web3Modal - you'll need to install this yourself
    import { Web3Modal } from '@web3modal/html'
    import { EthereumClient, w3mConnectors, w3mProvider } from '@web3modal/ethereum'
    import { walletContext } from '.';
    import { setContext } from 'svelte'
    
    // all this boilerplate is from the web3modal docs
    const chains = [mainnet, polygon]
    const projectId = import.meta.env.VITE_PROJECT_ID
    
    const { publicClient } = configureChains(chains, [w3mProvider({ projectId })])
    
    // except here we're using createConfig form this package instead of wagmi
    const wagmiConfig = createConfig({
      autoConnect: true,
      connectors: w3mConnectors({ projectId, chains }),
      publicClient
    })
    
    const ethereumClient = new EthereumClient(wagmiConfig, chains)
    
    let web3modal: Web3Modal
    
    // necessary if you're using SSR, because there's no window for the modal to attach to
    web3modal = new Web3Modal({ projectId }, ethereumClient)
    web3modal.setDefaultChain(polygon)

    setContext(walletContext, web3modal)
    </script>
    
    {#if web3modal}
        <Button on:click={() => web3modal.openModal()}>
        {#if $account?.isConnected}
            Disconnect
        {:else}
            Connect
        {/if}
        </Button>
    {/if}

    <slot />