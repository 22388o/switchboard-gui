<script>
 import Transfer from '$lib/Transfer.svelte';
 import Receive from '$lib/Receive.svelte';
 import { ethereum } from '$lib/stores.js';
 const { balance } = $ethereum;
</script>
<h1>Ethereum</h1>
<div class="container">
    <div class="item">
        <p>Balance: {balance} BTC</p>
    </div>
    <div class="item">
        <p>Send</p>
        <Transfer />
    </div>
    <div class="item">
        <p>Receive</p>
        <Receive chain="ethereum" />
    </div>
</div>

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
