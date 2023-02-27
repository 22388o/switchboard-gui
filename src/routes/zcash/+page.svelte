<script>
 import { invoke } from '@tauri-apps/api/tauri';
 import Transfer from '$lib/Transfer.svelte';
 import Receive from '$lib/Receive.svelte';
 import Console from '$lib/Console.svelte';
 import Amount from '$lib/Amount.svelte';
 import { zcash } from '$lib/stores.js';
 const { balance } = $zcash;

 let amount;
 let address;
 let cast_amount;
 async function send() {
     const method = 'sendtoaddress';
     const params = [address, amount];
     const response = await invoke('zcash', { method, params });
}

 async function z_listunspent() {
     const method = 'z_listunspent';
     const params = [];
     const response = await invoke('zcash', { method, params });
     const unspent = response.filter(unspent => unspent.amount > 0 && unspent.spendable).sort((a, b) => b.amount - a.amount);
     return unspent;
 }

 async function listunspent() {
     const method = 'listunspent';
     const params = [];
     const response = await invoke('zcash', { method, params });
     const unspent = response.filter(unspent => unspent.amount > 0 && unspent.spendable && !unspent.withdrawal).sort((a, b) => b.amount - a.amount);
     return unspent;
 }

 async function update() {
     t_unspent = await listunspent();
     meltable = t_unspent.map(unspent => unspent.amountZat).reduce((a, b) => a + b) / 100000000;
     z_unspent = await z_listunspent();
     castable = z_unspent.map(unspent => unspent.amountZat).reduce((a, b) => a + b) / 100000000;
 }

 async function melt(taddr) {
     const method = 'z_shieldcoinbase';
     const params = [taddr, z_unspent[0].address];
     console.log(params);
     const response = await invoke('zcash', { method, params });
     console.log(response);
 }

 let t_unspent = [];
 let z_unspent = [];
 let meltable = 0;
 let melting = 0;
 let castable = 0;
 let casting = 0;
</script>
<h1>Zcash</h1>
<div class="container">
    <div class="item">
        <p>Balance: <Amount value={balance}/></p>
    </div>
    <div class="item">
        <p>Send</p>
        <Transfer bind:address={address} bind:amount={amount} transfer={send}/>
    </div>
    <div class="item">
        <p>Receive</p>
        <Receive chain="zcash" />
    </div>
</div>
<div class="melt_cast">
    <div class="item">
        <p>Available to melt: <Amount value={meltable}/></p>
        <ul>
            {#each t_unspent as unspent ([unspent.txid, unspent.address])}
            <li>
                {unspent.address}
                <Amount value={unspent.amount} />
                <button on:click={() => melt(unspent.address)}>Melt</button>
            </li>
            {/each}
        </ul>
    </div>
    <div class="item">
        <p>Available to cast: <Amount value={castable}/></p>
        <p>Casting: <Amount value={casting}/></p>
        <label>
            Amount to cast:
        </label>
        <input type="number" bind:value={cast_amount} placeholder="0.001">
        <button>Cast</button>
        <ul>
            {#each z_unspent as unspent ([unspent.address])}
                <li>
                    {unspent.address}
                    <Amount value={unspent.amount} />
                </li>
            {/each}
        </ul>
    </div>
</div>
<Console chain="zcash" />
<style>
 .melt_cast {
     display: grid;
     grid-template-columns: 1fr 1fr;
     grid-auto-rows: 1fr;
     font-family: mono;
 }
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
