<script>
 import { invoke } from '@tauri-apps/api/tauri';
 import { afterUpdate } from 'svelte';

 export let chain;

 let history = [];
 let command = '';
 let index = -1;

 async function request(method, params) {
     let response;
     try {
         response = await invoke(`${chain}`, { method, params });
         response = JSON.stringify(response, null, 2);
     } catch(e) {
         console.log(e);
         response = e;
     }
     return response;
 }

 let history_element;

 async function enter(e) {
     console.log(e);
     if (e.code === 'ArrowUp') {
         command = history.at(index).input;
         index -= 1;
     }
     if (e.code === 'ArrowDown' && index < -1) {
         index += 1;
         command = history.at(index).input;
     }
     if (command.length > 0 && e.code === 'Enter') {
         index = -1;
         const terms = command.split(/\s+/);
         const method = terms[0];
         const params = terms.slice(1).map(param => {
             try {
                 return JSON.parse(param);
             } catch(e) {
                 return param;
             }
         });
         const output = await request(method, params);
         console.log(JSON.stringify(output, null, 2));
         history = [...history, {
             input: command,
             output: output,
         }];
         command = '';
     }
 }

 const scrollToBottom = async (node) => {
     node.scroll({ top: node.scrollHeight });
 };

 afterUpdate(() => {
     console.log("afterUpdate");
     if(history_element) scrollToBottom(history_element);
 });

</script>
<div class="console">
    <h1>{chain} Console</h1>
    <div bind:this={history_element} class="console-history">
        {#each history as command}
            <div class="console-input-log">
                {command.input}
            </div>
            <div class="console-output-log">
                <pre>{command.output}</pre>
            </div>
        {/each}
    </div>
    <input bind:value={command} on:keydown={enter} class="console-input" type="text" spellcheck="false">
</div>
<style>
 .console {
     border: solid;
     margin: 2px;
     display: flex;
     flex-direction: column;
 }

 .console-history {
     flex-grow: 1;
     box-sizing: border-box;
     padding: 4px;
     margin: 2px;
     overflow-y: auto;
 }

 .console-input-log,
 .console-output-log {
     box-sizing: border-box;
     padding: 6px 12px;
     margin: 2px;
 }

 .console-input-log:hover,
 .console-output-log:hover {
     background: #eee;
 }

 .console-input-log {
     font-weight: bold;
 }

 .console-input {
     flex-shrink: 0;
     box-sizing: border-box;
     padding: 12px;
     border: none;
     border-top: 1px solid;
     outline: none;
     background-color: #eee;
 }

 .console-input:hover,.console-input:focus {
     background-color: #ddd;
 }

 .console-history, .console-input {
     font-family: monospace;
 }

 input {
     width: 1fr;
     box-sizing: border-box;
 }
 .console {
     height: 400px;
 }
</style>
