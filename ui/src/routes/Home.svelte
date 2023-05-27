<script lang="ts">
</script>

<div class="prose text-white">
  <h2 class="text-white">
    This proof of concept is a simple Holochain application that allows users to
    mint NFTs and view their content inside this hApp.
  </h2>
  <p>
    The NFTs are minted on the Mumbai network, and the content is stored on
    Holochain.
  </p>
  <p>
    The main thing that we want to show is that even if users can arbitrarily
    mint NFTs based on arbitrary payloads in Holochain, Alice can never forge an
    NFT mint such that it appears to have been created by Bob.
  </p>
  <ul>
    <li>
      The ID of each NFT is a hash of (the content hash on Holochain + the
      author’s evm public key).
    </li>
    <li>
      On the Holochain side, each agent must bind their evm public key as the
      first entry in their source chain. Holochain uses this public key to
      produce a link to the content, where the link base hash is the NFT ID.
    </li>
    <li>
      On the EVM side, the ERC721 contract is given only the content hash as an
      argument for minting. The contract uses the sender's public key to produce
      a matching ID.
    </li>
    <li>
      When viewing the NFTs, the hApp uses the NFT ID to query Holochain for the
      content, and then decodes the content for display. It is possible to mint
      an NFT with no content on Holochain, but those NFTs can be filtered from
      view in the hApp.
    </li>
  </ul>
</div>
