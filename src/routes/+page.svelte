<script>
 import Console from '$lib/Console.svelte';
 import DepositsWithdrawals from '$lib/DepositsWithdrawals.svelte';
 import ZcashDepositsWithdrawals from '$lib/ZcashDepositsWithdrawals.svelte';
 import { onMount } from 'svelte';
 import { invoke } from '@tauri-apps/api/tauri';
 import { writeText } from '@tauri-apps/api/clipboard';

 async function generate(amount) {
     invoke('testchain', {
         method: 'refreshbmm',
         params: [0.0001],
     });
     invoke('bitassets', {
         method: 'refreshbmm',
         params: [0.0001],
     });
     invoke('zcash', {
         method: 'refreshbmm',
         params: [0.0001],
     });
     await invoke('mainchain', { method: 'generate', params: [1] });
 }

 async function launch_mainchain() {
     await invoke('spawn_mainchain');
 }

 async function stop_mainchain() {
     stop_testchain();
     stop_bitassets();
     stop_zcash();
     await invoke('mainchain', {
         method: 'stop',
         params: [],
     });
 }

 async function launch_testchain() {
     await invoke('spawn_testchain');
 }

 async function stop_testchain() {
     await invoke('testchain', {
         method: 'stop',
         params: [],
     });
 }

 async function launch_bitassets() {
     await invoke('spawn_bitassets');
 }

 async function stop_bitassets() {
     await invoke('bitassets', {
         method: 'stop',
         params: [],
     });
 }

 async function launch_zcash() {
     await invoke('spawn_zcash');
 }

 async function stop_zcash() {
     await invoke('zcash', {
         method: 'stop',
         params: [],
     });
 }

 async function launch_ethereum() {
     await invoke('spawn_ethereum');
 }

 async function stop_ethereum() {
     await invoke('kill_ethereum');
 }

 let mainchain_running = false;
 let testchain_running = false;
 let bitassets_running = false;
 let zcash_running = false;
 let ethereum_running = false;

 async function update() {
     invoke('mainchain', {
         method: 'getblockcount',
         params: [],
     }).then(() => { mainchain_running = true; })
       .catch(() => { mainchain_running = false; });
     invoke('testchain', {
         method: 'getblockcount',
         params: [],
     }).then(() => { testchain_running = true; })
       .catch(() => { testchain_running = false; });
     invoke('bitassets', {
         method: 'getblockcount',
         params: [],
     }).then(() => { bitassets_running = true; })
       .catch(() => { bitassets_running = false; });
     invoke('zcash', {
         method: 'getblockcount',
         params: [],
     }).then(() => { zcash_running = true; })
       .catch(() => { zcash_running = false; });
     invoke('web3', {
         method: 'eth_blockNumber',
         params: [],
     }).then(() => { ethereum_running = true; })
       .catch(() => { ethereum_running = false; });
 }

 let geth_console;
 onMount(async () => {
     setInterval(update, 1000);
     geth_console = await invoke('get_geth_console_command');
 });

 async function copyToClipboard() {
     await writeText(geth_console);
 }
</script>
<div>
    <ul>
        <li>
            {#if !mainchain_running}
                <button on:click={launch_mainchain}>Launch Mainchain Qt</button>
            {:else}
                <button on:click={stop_mainchain}>Stop Mainchain Qt</button>
            {/if}
        </li>
        {#if mainchain_running}
        <li>
            {#if !testchain_running}
                <button on:click={launch_testchain}>Launch Testchain Qt</button>
            {:else}
                <button on:click={stop_testchain}>Stop Testchain Qt</button>
            {/if}
        </li>
        <li>
            {#if !bitassets_running}
                <button on:click={launch_bitassets}>Launch Bitassets Qt</button>
            {:else}
                <button on:click={stop_bitassets}>Stop Bitassets Qt</button>
            {/if}
        </li>
        <!-- <li>
             {#if !zcash_running}
             <button on:click={launch_zcash}>Launch Zcash Daemon</button>
             {:else}
             <button on:click={stop_zcash}>Stop Zcash Daemon</button>
             {/if}
             </li> -->
        <li>
            {#if !ethereum_running}
                <button on:click={launch_ethereum}>Launch Ethereum Daemon</button>
            {:else}
                <button on:click={stop_ethereum}>Stop Ethereum Daemon</button>
            {/if}
        </li>
        {/if}
    </ul>
</div>
{#if mainchain_running}
<div class="mining">
    <h1> Mining </h1>
    <button on:click={() => generate(1000)}>Mine block</button>
</div>
{/if}
{#if testchain_running}
<DepositsWithdrawals sidechain={['testchain', 0]} />
{/if}
{#if bitassets_running}
<DepositsWithdrawals sidechain={['bitassets', 4]} />
{/if}
{#if zcash_running}
    <ZcashDepositsWithdrawals />
{/if}
{#if zcash_running}
    <Console chain={'zcash'} />
{/if}
{#if ethereum_running}
    <h1>Ethereum</h1>
    <p>Deposit and withdrawal GUI is currently not supported for Ethereum, use
    geth console instead.</p>
    <p>To open geth console run <button on:click={copyToClipboard} title="Copy to clipboard">{geth_console}</button> in your terminal.</p>
{/if}

<style>
 .mining {
     width: 1fr;
     border: solid;
     margin: 2px;
 }
 a {
     border: solid;
     display: block;
     margin: 2px;
     padding: 10px;
     text-align: center;
     background-color: #eee;
     color: #000;
     text-decoration: none;
 }
</style>
