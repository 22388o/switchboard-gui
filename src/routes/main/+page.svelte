<script>
 import { invoke } from '@tauri-apps/api/tauri';
 import Transfer from '$lib/Transfer.svelte';
 import Receive from '$lib/Receive.svelte';
 import Console from '$lib/Console.svelte';
 import { main } from '$lib/stores.js';

 let amount;
 let address;
 async function send() {
     const method = 'sendtoaddress';
     const params = [address, amount];
     const response = await invoke('main_request', { method, params });
     console.log(response);
}
</script>
<h1>Main</h1>
<div class="container">
    <div class="item">
        <p>Balance: {$main.balance / 100000000} BTC</p>
    </div>
    <div class="item">
        <p>Send</p>
        <Transfer bind:address={address} bind:amount={amount} transfer={send}/>
    </div>
    <div class="item">
        <p>Receive</p>
        <Receive chain="main" />
    </div>
</div>
<Console chain="main" />
<style>
 .container {
     display: grid;
     grid-template-columns: 1fr 1fr 1fr;
     grid-auto-rows: 1fr;
 }

 .item {
     border: solid;
     margin: 2px;
     padding: 10px;
 }
</style>
