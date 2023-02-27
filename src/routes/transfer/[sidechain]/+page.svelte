<script>
 import { invoke } from '@tauri-apps/api/tauri';
 import Amount from '$lib/Amount.svelte';
 import Transfer from '$lib/Transfer.svelte';
 import * as stores from '$lib/stores.js';

 export let data;
 const sidechain = stores[data.sidechain];
 let withdraw_amount;
 let deposit_amount;

 async function withdraw(sidechain, amount) {
     const fee = 0.0001;
     const method = 'withdraw';
     const params = [amount, fee];
     await invoke(sidechain, { method, params});
 }

 async function deposit(sidechain, amount) {
     const fee = 0.0001;
     const method = 'deposit';
     const params = [amount, fee];
     await invoke(sidechain, { method, params });
 }
</script>

<div>
    <h1>{$sidechain.name}</h1>
    <div class="container">
        <div class="item">
            <input type="number" bind:value={withdraw_amount} placeholder="Withdrawal amount">
            <button on:click={() => withdraw(data.sidechain, withdraw_amount)}>Withdraw</button>
        </div>
        <div class="item">
            <Amount value={$sidechain.refundable} /> refundable
        </div>
        <div class="item">
            <input type="number" bind:value={deposit_amount} placeholder="Deposit amount">
            <button on:click={() => deposit(data.sidechain, deposit_amount)}>Deposit</button>
        </div>
    </div>
</div>

<style>
 .container {
     display: grid;
     grid-template-columns: 1fr 1fr 1fr;
 }

 .item {
     border: solid;
     margin: 2px;
     padding: 10px;
 }
</style>
