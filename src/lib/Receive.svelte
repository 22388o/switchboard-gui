<script>
 import { onMount } from 'svelte';
 import { invoke } from '@tauri-apps/api/tauri';
 import { writeText } from '@tauri-apps/api/clipboard';
 export let chain;
 export let address;


 onMount(async () => {
     await newaddress();
 });

 async function newaddress() {
     if (chain === 'main') {
         address = await invoke('mainchain', { method: 'getnewaddress', params: [] });
     } else if (chain === 'zcash') {
         address = await invoke('zcash', { method: 'getnewaddress', params: [] });
     }
 }
 async function copyToClipboard() {
     await writeText(address);
 }
</script>

<form class="container">
    <button on:click={copyToClipboard} title="Copy to clipboard">{address}</button>
    <button class="transfer" on:click={newaddress}>
        New address
    </button>
</form>

<style>
 .container {
     display: grid;
     grid-template-rows: 1fr 1fr 1fr;
 }
 button {
     width: 100%;
 }
</style>
