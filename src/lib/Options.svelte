<script lang="ts">
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/tauri';
    import { appConfigDir } from '@tauri-apps/api/path';
    import { is_open } from './store';


    function close() {
        is_open.set(false);
    }

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

    let ip:string[] = ['', '', '', ''];
    let port:string = '';
    let resp:string = '';

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

    async function save() {
        console.log(ip, port);
        const ipString = ip.join('.');
        const jsonData = { ip4: ipString, port }; // Updated: Changed 'ip' to 'ip4'
        const jsonString = JSON.stringify(jsonData);
        console.log(jsonString);
        
        resp = await invoke('change_config', {  configAsJsonString: jsonString});
        close();
    }

    onMount(() => {
        const inputElements = document.querySelectorAll('.ip-input');

        inputElements.forEach((input, index) => {
        input.addEventListener('input', (event) => {
            const inputElement = event.target as HTMLInputElement;

            const inputValue = inputElement.value;

            if (inputValue.length > 3) {
                inputElement.value = inputValue.slice(0, 3);
            }

            ip[index] = inputElement.value;
          if (index < inputElements.length - 1 && inputValue.length === 3) {
              const nextInputElement = inputElements[index + 1] as HTMLInputElement;
              nextInputElement.focus();
          }
        });
        });
    });

    let cMode: string;
    const cookie = document.cookie;
    if (cookie) {
      const cookieValue = cookie.split(";").find((c) => c.trim().startsWith("cMode="))?.split("=")[1];
      cMode = cookieValue === "Easy" ? "Dev" : "Easy";
    }

    function changeMode(){
        cMode = cMode === "Easy" ? "Dev" : "Easy"
        document.cookie = `cMode=${cMode};max-age=31536000;path=/`
    }
</script>
<div>
  <h1>Options</h1>
  <li><h3 on:click={changeMode}>Mode: {cMode}</h3></li>
  <h2 class="discription">IP-Adresse</h2>
  <div class="ip-row">
    {#each ip as segment, index}
      <input class="ip-input" type="text" bind:value={segment} maxlength="3" />
      {#if index < 3}
        <span class="separator">.</span>
      {/if}
    {/each}
  </div>
  <h2 class="discription">Port</h2>
  <input type="text" bind:value={port} />

  <button on:click={close} class="modal-buttons modal-buttons-cancel">Cancel</button>
  <button on:click={save} class="modal-buttons modal-buttons-save">Save</button>
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
    flex-direction: row;
    justify-content: center;
  }
  
  .ip-input {
    width: 2em;
    height: 100%;
    font-size: 2rem;
    text-align: center;
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
  .separator {
    margin: 0 5px;
    font-size: 2rem;
  }
  .discription{
    font-size: 2rem;
    margin-top: 0.5em;
  }

     .modal-buttons {
        margin-top: 10px;
        width: 100px;
        height: 30px;
        border-radius: 5px;
        border: none;
        background-color: var(--overlay0);
        color: var(--text);
        font-size: 16px;
        z-index: 10000;
        cursor: pointer;
    }
    .modal-buttons-save:hover {
        background-color: var(--green);
        color: var(--crust);
    }
    .modal-buttons-cancel:hover {
        background-color: var(--red);
        color: var(--crust);
    }
</style>
