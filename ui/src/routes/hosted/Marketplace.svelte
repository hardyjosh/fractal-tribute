<script lang="ts">
  import { Heading } from "flowbite-svelte";
  import GameStats from "$lib/components/GameStats.svelte";
  import AllNfts from "$lib/components/AllNfts.svelte";
  import En from "ui/src/lib/components/i18n/En.svelte";
  import Tr from "ui/src/lib/components/i18n/Tr.svelte";
</script>

<div class="flex flex-row gap-x-8 pt-8 items-center">
  <div class="w-full md:mt-12">
    <GameStats />
  </div>
</div>
<div class="w-full flex flex-col gap-y-6">
  <Heading tag="h3" class="mt-14"
    ><En>Snapshot marketplace</En><Tr>Görüntü pazarı</Tr></Heading
  >
  <span class="text-xl"
    ><En
      >By minting another player's snapshot you are pushing them up the
      leaderboard!</En
    ><Tr
      >Başka bir oyuncunun oluşturduğu görüntüyü basarak (mintleyerek) onu
      liderlik tablosunda yukarılara taşıyorsunuz!
    </Tr></span
  >
  <AllNfts wrap />
</div>
