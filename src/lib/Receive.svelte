<script>
 import { invoke } from '@tauri-apps/api/tauri';
 export let chain;
 export let address;

 async function newaddress() {
   address = await invoke("get_new_address", { chain });
 }
</script>

<form class="container">
    <input type="text" bind:value={address} placeholder="address">
    <button class="transfer" on:click={newaddress}>
        New address
    </button>
</form>

<style>
 .container {
     display: grid;
     grid-template-rows: 1fr 1fr 1fr;
 }
</style>
