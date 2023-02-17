<script>
 import Console from '$lib/Console.svelte';
 import { invoke } from '@tauri-apps/api/tauri';
 import { writeText } from '@tauri-apps/api/clipboard';
 import { ethereum } from '$lib/stores.js';
 import { onMount } from 'svelte';

 const { balance } = $ethereum;


 let geth_console;
 onMount(async () => {
     geth_console = await invoke('get_geth_console_command');
 });
 async function copyToClipboard() {
     await writeText(geth_console);
 }
</script>
<h1>Ethereum</h1>
<p>To open Geth JSRE call <button on:click={copyToClipboard} title="Copy to clipboard">{geth_console}</button> in the terminal.</p>
