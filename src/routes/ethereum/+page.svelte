<script>
 import Console from '$lib/Console.svelte';
 import { ethereum } from '$lib/stores.js';
 const { balance } = $ethereum;
</script>
<h1>Ethereum</h1>
<div class="container">
    <div class="item">
        <p>Balance: {balance / 100000000} BTC</p>
    </div>
</div>
<Console chain="ethereum" />

<style>
 .container {
     display: grid;
     grid-template-columns: 1fr;
     grid-auto-rows: 1fr;
 }

 .item {
     border: solid;
     margin: 2px;
     padding: 10px;
 }
</style>
