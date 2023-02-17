<script>
 import Amount from '$lib/Amount.svelte';
 import { main, zcash, ethereum } from '$lib/stores.js';
</script>

<div class="panel">
    <a class="main" href="/main">
        <h1>
            Main
        </h1>
        <p>Balance: <Amount value={$main.balance} /> BTC</p>
        <p>Block count: {$main.block_count}</p>
    </a>
    <div class="container">
        {#each [ $zcash, $ethereum ] as sidechain}
            <a class="transfer" href="/transfer/{sidechain.id}">
                <Amount value={sidechain.refundable } /> BTC refundable
            </a>
            <a class="sidechain" href="/{sidechain.id}">
                <h1>
                    {sidechain.name}:
                </h1>
                <p>
                    Balance: <Amount value={sidechain.balance }/> BTC
                </p>
                <p>
                    Block count: {sidechain.block_count}
                </p>
            </a>
        {/each}
    </div>
</div>

<style>
 .mining {
     width: 1fr;
     border: solid;
     margin: 2px;
 }
 .main {
     width: 1fr;
     border: solid;
     margin: 2px;
 }
 .container {
     display: grid;
     grid-template-columns: 2fr 1fr;
     grid-auto-rows: 1fr;
 }
 .transfer {
     border: solid;
     padding: 10px;
     margin: 2px;
 }
 .panel {
     display: grid;
     grid-template-columns: 1fr 3fr;
 }
 .sidechain {
     border: solid;
     margin: 2px;
 }
 a {
     display: block;
     padding: 10px;
     text-align: center;
     background-color: #eee;
     color: #000;
     text-decoration: none;
 }
 a:hover {
     background-color: #fff;
 }
</style>
