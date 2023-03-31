<script lang="ts">
  import { connectWallet, disconectWallet } from "./";
</script>

<button on:click={connectWallet}>Connect Wallet</button>

<button on:click={disconectWallet}>Disconnect Wallet</button>
