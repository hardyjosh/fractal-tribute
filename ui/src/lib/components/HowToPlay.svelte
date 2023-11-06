<script lang="ts">
  import { fade } from "svelte/transition";
  import CreateEvmKeyBinding from "$lib/components/CreateEvmKeyBinding.svelte";
  import { happ } from "$lib/stores";
  import { A, Button, Heading, Input } from "flowbite-svelte";
  import { createEventDispatcher } from "svelte";
  import singlelogo from "$lib/assets/single-logo.svg";
  import mintsnapshot from "$lib/assets/mint-snapshot.svg";
  import howtoplay1 from "$lib/assets/how-to-play-1.svg";
  import howtoplay4 from "$lib/assets/how-to-play-4.svg";
  import En from "$lib/components/i18n/En.svelte";
  import Tr from "$lib/components/i18n/Tr.svelte";

  export let hasEvmAddress: boolean;

  const dispatch = createEventDispatcher();

  let howToPlayStep = 0;
  let name, badNameCharLength;

  $: if (name) {
    badNameCharLength = name.length < 2 || name.length > 15;
  }

  const inc = () => {
    howToPlayStep++;
  };

  const completeOnboarding = async () => {
    const record = await $happ.createProfile({ name });
    dispatch("onboarding-complete");
  };
</script>

<div
  class="flex flex-col justify-between items-center gap-y-6 max-w-[650px] min-h-[500px] p-6"
>
  {#if howToPlayStep === 0}
    <div in:fade class="flex flex-col gap-y-5 text-center items-center">
      <img class="w-32" src={singlelogo} alt="fractal tribute logo" />
      <Heading tag="h2">
        <En>Welcome to Fractal Tribute!</En>
        <Tr>Fractal Tribute'e hoş geldin!</Tr>
      </Heading>
      <Heading tag="h6" class="font-semibold"
        ><En
          >A peer-to-peer fully distributed, collaborative artistic NFT game.</En
        >
        <Tr
          >Eşler arası (P2P) tamamen dağıtık, işbirliğine dayalı sanatsal bir
          NFT oyunu.</Tr
        ></Heading
      >
      <p>
        <En>
          This game began at 10am PDT on Thursday, November 2nd and ends at 10am
          PDT on Tuesday November 7th.</En
        >
        <Tr>
          Bu oyun 2 Kasım Perşembe günü sabah 10'da (PDT) başladı ve 7 Kasım
          Salı günü sabah 10'da (PDT) sona erecek.
        </Tr>
      </p>
      <p>
        <En>
          Note: If you are joining after the game starts you may want to wait
          10-30 minutes until the full artwork syncs with all the other creator
          contributions before beginning to make moves.
        </En>
        <Tr
          >Not: Oyun başladıktan sonra katılıyorsanız, hamle yapmaya başlamadan
          önce üretilen tüm sanatsal görüntülerin diğer tüm içerik
          oluşturucuların katkılarıyla senkronize olmasını sağlamak için 10-30
          dakika beklemeniz iyi olabilir.</Tr
        >
      </p>
    </div>
    <Button class="bg-fractalorange border-2 border-black" on:click={inc}
      >Next: How to Play</Button
    >
  {:else if howToPlayStep === 1}
    <div in:fade class="flex flex-col gap-y-5 text-center items-center">
      <img class="w-80" src={howtoplay1} alt="" />
      <p>
        <En>
          Creators make moves by placing pixels with colors and patterns… 20
          placements can be made in each move.
        </En>
        <Tr
          >Oyuncular/oluşturucular pikselleri renk ve desenlerle yerleştirerek
          hamle yaparlar... Her hamlede 20 yerleştirme yapılabilir.
        </Tr>
      </p>
      <p>
        <En>
          NFTs are minted along the way, whenever a creator thinks their move is
          worthy of a snapshot.
        </En>
        <Tr
          >NFT'ler, bir oyuncunun, hamlesinin anlık görüntü almaya değer
          olduğunu düşündüğü herhangi bir anda basılabilir.</Tr
        >
      </p>
    </div>
    <Button class="bg-fractalorange border-2 border-black" on:click={inc}
      >Next: About the NFTs</Button
    >
  {:else if howToPlayStep === 2}
    <div in:fade class="flex flex-col gap-y-5 text-center items-center">
      <img class="w-80" src={mintsnapshot} alt="mint snapshot" />
      <p>
        <En>
          After a creator mints a snapshot, others can mint them as well,
          indicating which snapshots are their favorites in the series.
        </En>
        <Tr
          >Bir içerik oluşturucu bir görüntüyü bastıktan (mintledikten) sonra,
          diğerleri de bunları basabilir ve hangi görüntülerin serideki
          favorileri olduğunu belirtebilir.
        </Tr>
      </p>
      <p>
        <En>
          All NFTs are created and saved/stored in the Holochain game app and
          minted on the Polygon network. A hosted marketplace of the NFT
          snapshots will be available for public minting at <A
            href="https://fractal-tribute.com">fractal-tribute.com</A
          >.
        </En>
        <Tr
          >Tüm NFT'ler Holochain oyun uygulamasında oluşturulur,
          kaydedilir/depolanır ve Polygon ağında basılır (mintlenir).
          NFT’leştirilmiş görüntüleri barındıran bir pazar olan
          <A href="https://fractal-tribute.com">fractal-tribute.com</A> adresinde
          bu NFT’ler herkese açık olarak basılabilecektir.</Tr
        >
      </p>
    </div>
    <Button class="bg-fractalorange border-2 border-black" on:click={inc}
      >Next: How Claims Work</Button
    >
  {:else if howToPlayStep === 3}
    <div in:fade class="flex flex-col gap-y-5 text-center items-center">
      <img class="w-80" src={howtoplay4} alt="" />
      <p>
        <En>Sale of game NFTs will close 24 hours after the game ending.</En><Tr
          >Oyun NFT'lerinin satışı, oyun sona erdikten 24 saat sonra
          kapanacaktır.
        </Tr>
      </p>
      <p>
        <En>
          Following that, creators can return to <A
            href="https://fractal-tribute.com">fractal-tribute.com</A
          > or the installed game to claim their fraction of the NFT minting fees
          based on their contributions.
        </En>
        <Tr
          >Bunu takiben, içerik oluşturucular <A
            href="https://fractal-tribute.com">fractal-tribute.com</A
          > adresine veya yüklenmiş oyuna geri dönerek katkılarına göre NFT basım
          ücretlerinin kendi paylarına düşen kısmını talep edebilirler.</Tr
        >
      </p>
    </div>
    {#if !hasEvmAddress}
      <Button class="bg-fractalorange border-2 border-black" on:click={inc}
        ><En>Next: Get Started!</En><Tr>Sıradaki: Haydi Başlayalım!</Tr></Button
      >
    {:else}
      <Button
        class="bg-fractalorange border-2 border-black"
        on:click={completeOnboarding}>Ok, I'm ready to play!</Button
      >
    {/if}
  {:else if howToPlayStep === 4}
    <div in:fade class="flex flex-col gap-y-5 text-center">
      <Heading tag="h4"
        ><En>Ok, let's get you set up to play.</En><Tr
          >Tamam, haydi seni oyuna hazırlayalım.</Tr
        ></Heading
      >
    </div>
    <div class="flex flex-col gap-y-5 text-center">
      <Heading tag="h5"><En>What is your name?</En><Tr>Adın ne?</Tr></Heading>
      <span
        ><En>Choose carefully, you can't change this later!</En><Tr
          >Dikkatli seç çünkü daha sonra bunu değiştiremeyeceksin!</Tr
        ></span
      >
      <Input defaultClass="text-center" type="text" bind:value={name} />
      {#if badNameCharLength}
        <p transition:fade class="text-red-500">
          Name must be between 2 and 15 characters
        </p>
      {/if}
    </div>
    <Button
      disabled={!name || badNameCharLength}
      class="bg-fractalorange border-2 border-black"
      on:click={inc}
      ><En>Create my Profile</En><Tr>Profilimi oluştur</Tr>
    </Button>
  {:else if howToPlayStep === 5}
    <CreateEvmKeyBinding on:evmKeyBindingCreated={inc} />
  {:else if howToPlayStep === 6}
    <div in:fade class="flex flex-col gap-y-5 text-center">
      <Heading tag="h4"
        ><En>That's it! Be creative and have fun 💫</En><Tr
          >İşte bu kadar! Yaratıcı ol ve tadını çıkar 💫</Tr
        ></Heading
      >
      <p>
        <En>
          To see these instructions again, click on "how to play" on the top
          right.</En
        ><Tr
          >Bu talimatları tekrar görmek istersen sağ üstte yer alan “Nasıl
          oynanır” butonuna tıklayabilirsin.</Tr
        >
      </p>
    </div>
    <Button
      class="bg-fractalorange border-2 border-black"
      on:click={completeOnboarding}>I'm ready!</Button
    >
  {/if}
</div>
