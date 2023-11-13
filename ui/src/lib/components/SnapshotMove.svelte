<script lang="ts">
  import { fade } from "svelte/transition";
  import { onMount, createEventDispatcher } from "svelte";
  import { account, network } from "svelte-wagmi-stores";
  import { type ActionHash } from "@holochain/client";
  import { happ, nftContract, web3modal } from "$lib/stores";
  import { Button, Spinner, Heading, Alert } from "flowbite-svelte";
  import {
    hexToBigInt,
    keccak256,
    type Address,
    parseEther,
    type Hex,
  } from "viem";
  import { snapshotEvaluable } from "$lib/helpers";
  import { Confetti } from "svelte-confetti";
  import En from "$lib/components/i18n/En.svelte";
  import Tr from "$lib/components/i18n/Tr.svelte";

  const dispatch = createEventDispatcher();

  export let isPostMove: boolean = false;
  export let open: boolean;

  export let move: ActionHash;
  $: _move = hexToBigInt(keccak256(move));

  const price = parseEther("0");

  let key: Address;
  $: mismatchingKey = $account?.address && key && $account?.address !== key;
  $: wrongNetwork =
    $account?.address && $happ.dnaProperties.chainId !== $network?.chain?.id;

  onMount(async () => {
    key = await $happ.getEvmAddress();
  });

  let hash: Hex;

  $: ({ write, status, error } = $nftContract.write({
    functionName: "flow",
    args: [snapshotEvaluable, [_move], []],
    onSuccess: ({ hash: _hash }) => {
      hash = _hash;
      dispatch("snapshotMinted", hash);
    },
  }));

  const snapshotMove = async () => {
    await $happ.createTokenIdForGameMove(move);
    await write();
  };

  $: console.log($error);
</script>

{#if ($status == "idle" || $status == "error") && !hash}
  <div in:fade class="flex flex-col justify-center gap-y-4">
    {#if isPostMove}
      <Heading tag="h4">Nice move!</Heading>
    {:else}
      <Heading tag="h4"
        ><En>Create snapshot</En><Tr>Anlık görüntü al</Tr></Heading
      >
    {/if}
    <p>
      <En>Creating a snapshot is free, you'll just need to pay the gas fee.</En
      ><Tr
        >Anlık görüntü almak ücretsizdir, sadece aktarım ücreti (gas fee) ödemen
        yeterli.
      </Tr>
    </p>
    <p>
      <En>
        One you have created your snapshot, other players (and the public) will
        be able to mint your snapshot to push it up the leaderboard.</En
      ><Tr
        >Anlık görüntü aldığında, diğer oyuncular (ve herkes) aldığın görüntüyü
        basabilir (mintleyebilir) ve böylece görüntünün liderlik panosunda yer
        almasını sağlayabilir.</Tr
      >
    </p>
    <p>
      <En>
        The MATIC collected will be sent to the game pool to be redistributed to
        players at the end of the game</En
      ><Tr
        >Elde edilen MATIC, oyunun sonunda oyunculara tekrar dağıtılmak üzere
        oyun havuzuna gönderilecek.
      </Tr>
    </p>
    <p>
      <En>
        Your wallet must be connected to the Polygon network to create a
        snapshot.</En
      ><Tr
        >Anlık görüntü alabilmek için cüzdanının Polygon ağına bağlı olması
        gerekmektedir.</Tr
      >
    </p>
    <div class="flex gap-x-2">
      <Button
        class="border-2 border-black"
        color="none"
        on:click={() => {
          open = false;
        }}><En>Maybe later</En><Tr>Belki daha sonra</Tr></Button
      >
      {#if $account?.isConnected}
        <Button
          class="bg-fractalorange border-2 border-black"
          disabled={mismatchingKey || wrongNetwork}
          on:click={snapshotMove}
        >
          <En>Mint snapshot</En><Tr>Görüntüyü bas (mintle)</Tr>
        </Button>
      {:else}
        <Button
          class="bg-fractalorange border-2 border-black self-start"
          on:click={() => {
            $web3modal.openModal();
          }}><En>Connect wallet</En><Tr>Cüzdan bağla</Tr></Button
        >
      {/if}
    </div>
    {#if $status == "error"}
      <p transition:fade class="text-red-500">
        {$error?.details || $error?.shortMessage || $error}
      </p>
    {/if}
    {#if wrongNetwork}
      <Alert>
        <p>
          <En>
            You're connected to the wrong network. Please switch to the{" "}
            {$happ.dnaProperties.chainId === 137 ? "Polygon" : "Mumbai"}{" "}
            network.</En
          ><Tr
            >Yanlış ağa bağlandınız. Lütfen{" "}
            {$happ.dnaProperties.chainId === 137
              ? "Polygon"
              : "Mumbai"}{" "}ağına bağlanınız.</Tr
          >
        </p>
      </Alert>
    {:else if mismatchingKey}
      <Alert>
        <p>
          <En>
            You previously bound the Ethereum wallet {key} to your Holochain agent
            key.</En
          ><Tr
            >Daha önce Ethereum cüzdanını {key} Holochain anahtarınıza bağladınız.</Tr
          >
        </p>
        <p>
          <En>
            You'll need switch to this account in your wallet before you can
            mint.</En
          ><Tr
            >Görüntüyü basabilmeniz (mintleyebilmeniz) için cüzdanınızda bu
            hesaba geçiş yapmanız gerekir.</Tr
          >
        </p>
      </Alert>
    {/if}
  </div>
{:else if $status === "loading"}
  <div in:fade class="flex flex-col items-center gap-y-4 my-12">
    <Spinner size="10" class="mr-2" />
    <Heading tag="h4" class="text-center"
      ><En>Minting snapshot</En><Tr>Görüntüyü basıyor (mintliyor)</Tr></Heading
    >
    <span
      ><En>Please check your wallet to confirm</En><Tr
        >Teyit etmek için lütfen cüzdanını kontrol et</Tr
      ></span
    >
  </div>
{:else if $status === "success" || hash}
  <div class="fixed inset-0 translate-x-1/2 pointer-events-none">
    <Confetti
      x={[-4, 4]}
      y={[0, 1]}
      fallDistance="1500px"
      amount={500}
      cone={true}
      delay={[0, 700]}
    />
  </div>
  <div in:fade class="flex flex-col items-center gap-y-4 my-12">
    <div class="rounded-full bg-gray-100 p-6 text-4xl">🎉</div>
    <Heading class="text-center" tag="h4"
      ><En>Minting successful!</En><Tr>Basma (mintleme) işlemi başarılı!</Tr
      ></Heading
    >
    <p>
      <En>Snapshot minted!</En><Tr>Görüntü basıldı!</Tr>
      <a
        href={`${$network.chain.blockExplorers.default.url}/tx/${hash}`}
        target="_blank"
        class="underline"
      >
        <En>View on explorer</En><Tr>Kaşif (Explorer) üzerinde görüntüle</Tr></a
      >
    </p>
    <Button
      class="bg-fractalorange border-2 border-black"
      on:click={() => {
        open = false;
      }}><En>Done</En><Tr>Tamam</Tr></Button
    >
  </div>
{/if}
