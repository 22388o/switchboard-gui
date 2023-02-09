<script>
 import { main, zcash, ethereum } from '$lib/stores.js';
 import { invoke } from '@tauri-apps/api/tauri';
 import { listen, emit } from '@tauri-apps/api/event';
 import { onMount } from 'svelte';
 import '../reset.less';
 import '../app.less';

 onMount(async () => {
   setInterval(async () => {
     const balances = await invoke('get_balances');
     const block_counts = await invoke('get_block_counts');
     main.update(main => ({ balance: balances.main, block_count: block_counts.main }));

     zcash.update(zcash => ({
       ...zcash,
       balance: balances.zcash,
       refundable: balances.zcash_refundable,
       block_count: block_counts.zcash,
     }));

     ethereum.update(ethereum => ({
       ...ethereum,
       balance: balances.ethereum,
       refundable: balances.ethereum_refundable,
       block_count: block_counts.ethereum,
     }));
   }, 1000);
 });

 async function generate(amount) {
   await invoke("generate", { amount });
 }
</script>
<a href="/">Board</a>
<slot />
<div class="mining">
    <h1> Mining </h1>
    <button on:click={() => generate(1000)}>Mine block</button>
</div>

<style>
 .mining {
     width: 1fr;
     border: solid;
     margin: 2px;
 }
</style>
