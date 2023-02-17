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
   address = await invoke("get_new_address", { chain });
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
