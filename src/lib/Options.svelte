<script lang="ts">
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/tauri';
    import { appConfigDir } from '@tauri-apps/api/path';
    

    let realConfDir = '';
    appConfigDir()
    .then(appConfigDirPath => {
        realConfDir = appConfigDirPath+"peachy-pager/config.json";
        realConfDir = realConfDir.replace("com.maxi.peachypager/", "");
        console.log("Directory path:", realConfDir);
    })
    .catch(error => {
        console.error("Error:", error);
    });

    let ip = ['', '', '', ''];
    let port = '';
    let resp = '';

    fetch(realConfDir)
    .then(response => response.json())
    .then(data => {
      // Assuming the JSON structure is an array with two objects
        if (Array.isArray(data) && data.length === 2) {
            // Extract the values and assign them to variables
            ip = data[0].ip || ip;
            port = data[1].port || port;
        }
    })
    .catch(error => {
        console.error('Error loading JSON:', error);
    });

    async function first_time_file() {
        console.log(ip, port);
        const ipString = ip.join('.');
        const jsonData = { ip4: ipString, port }; // Updated: Changed 'ip' to 'ip4'
        const jsonString = JSON.stringify(jsonData);
        console.log(jsonString);
        
        resp = await invoke('change_config', {  configAsJsonString: jsonString});
    }

    onMount(() => {
        const inputElements = document.querySelectorAll('.ip-input');

        inputElements.forEach((input, index) => {
        input.addEventListener('input', (event) => {
            const inputValue = event.target.value;

            if (inputValue.length > 3) {
            event.target.value = inputValue.slice(0, 3);
            }

            ip[index] = event.target.value;

            if (index < inputElements.length - 1 && inputValue.length === 3) {
            inputElements[index + 1].focus();
            }
        });
        });
    });
</script>

<div>
  <h1>Options</h1>

  <div class="ip-row">
    {#each ip as segment, index}
      <input class="ip-input" type="text" bind:value={segment} maxlength="3" />
      {#if index < 3}
        <span class="separator">.</span>
      {/if}
    {/each}
  </div>

  <input type="text" bind:value={port} />

  <button class="temp-save" on:click={first_time_file}>temp save</button>
  <h3> - {resp} - </h3>
</div>

<style>
  div {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .separator {
    margin: 0 5px;
  }
  
  .ip-row {
    display: flex;
    align-items: center;
  }
  
  .ip-input {
    width: 2em;
    height: 100%;
    font-size: 2rem;
    text-align: center;
    margin-top: 0.5em;
    background-color: var(--surface0);
    border: 1px solid var(--surface1);
    border-radius: 0.5em;
    box-shadow: 0 0 0 0.1em var(--surface1) inset;
  }
  .temp-save{
    width: 60%;
    height: 100%;
    font-size: 2rem;
    text-align: center;
    margin-top: 0.5em;
    background-color: var(--surface0);
    border: 1px solid var(--surface1);
    border-radius: 0.5em;
    box-shadow: 0 0 0 0.1em var(--surface1) inset;
    cursor: pointer;
    transition: all 0.2s ease-in-out;
  }
  .temp-save:hover{
    box-shadow: 0 0 10px var(--surface2);
  }
  .temp-save:active {
      background-color: var(--surface1);
  }
</style>
