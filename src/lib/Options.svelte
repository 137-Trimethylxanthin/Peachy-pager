<script lang="ts">
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/tauri';
    import { appConfigDir } from '@tauri-apps/api/path';
    import { is_open } from './store';


    function close() {
        is_open.set(false);
    }

    let ip:string[] = ['', '', '', ''];
    let port:string = '';
    let resp:string = '';



    async function setSettings() {
        let jsonString:string = await invoke('get_json_data');
        console.log(jsonString);
        let jsonData = JSON.parse(jsonString);
        console.log(jsonData);
        ip = jsonData.ip4.split('.');
        port = jsonData.port;
    }

    setSettings();

    async function save() {
        resp = '';
        console.log(ip, port);
        if (ip.some((segment) => segment === '')) {
          resp = 'Bitte eine IP angeben ';
        }
        if (port === '') {
          if (resp !== '') {
            resp += 'und ';
          }
          resp += 'Bitte einen Port angeben ';
        }
        if (resp !== '') {
          return;
        }

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
                nextInputElement.select();
            }

            if (index > 0 && inputValue.length === 0) {
                const previousInputElement = inputElements[index - 1] as HTMLInputElement;
                previousInputElement.focus();
            }
          });
        });
    });

  let cMode: string = localStorage.getItem('cMode') || "Easy";

  function changeMode(){
      cMode = cMode === "Easy" ? "Dev" : "Easy"
      localStorage.setItem('cMode', cMode);
  }

  function selectInput(event: any) {
    event.target.select();
  }

function handleKeyPress(event: any) {
    const inputElement = event.target as HTMLInputElement;
    const inputValue = inputElement.value;
    const key = event.key;
    console.log(key);
    if ((key < '0' || key > '9') && key !== ' ' && key !== 'Enter' && key !== 'Backspace') {
        event.preventDefault();
    } else if ((key === ' ' || key === 'Enter') && key !== 'Backspace') {
        event.preventDefault();
        const inputElements = document.querySelectorAll('.ip-input');
        const index = Array.from(inputElements).indexOf(inputElement);
        if (index < inputElements.length - 1) {
            const nextInputElement = inputElements[index + 1] as HTMLInputElement;
            nextInputElement.focus();
            nextInputElement.select();
        }
    } else if (key === 'Backspace') {
        const inputElements = document.querySelectorAll('.ip-input');
        const index = Array.from(inputElements).indexOf(inputElement);
        if (index > 0 && inputValue.length === 0) {
            const previousInputElement = inputElements[index - 1] as HTMLInputElement;
            previousInputElement.focus();
        }
    }
}
</script>
<div>
  <h1>Options</h1>
  <h3 class="mode" on:click={changeMode}>Mode: {cMode}</h3>
  <h2 class="discription">IP-Adresse</h2>
  <div class="ip-row">
    {#each ip as segment, index}
      <input class="ip-input" type="text" bind:value={segment} maxlength="3" on:click={selectInput} on:keydown={handleKeyPress}/>
      {#if index < 3}
        <span class="separator">.</span>
      {/if}
    {/each}
  </div>
  <h2 class="discription">Port</h2>
  <input type="text" class="port" bind:value={port} on:click={selectInput} on:keypress={handleKeyPress}/>

  <p class="resp">{resp}</p>

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

  .port{
    width: 5em;
    height: 100%;
    font-size: 2rem;
    text-align: center;
    background-color: var(--surface0);
    border: 1px solid var(--surface1);
    border-radius: 0.5em;
    box-shadow: 0 0 0 0.1em var(--surface1) inset;
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

  .separator {
    margin: 0 5px;
    font-size: 2rem;
  }
  .discription{
    font-size: 2rem;
    margin-top: 0.5em;
    font-weight: 500;
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
    h3{
      background-color: var(--surface1);
      color: var(--text);
      font-size: 1.1rem;
      font-weight: 500;
      padding: 0.5em 1em;
      border-radius: 0.5em;
      box-shadow: 0 0 0 0.1em var(--surface2) inset;
      cursor: pointer;    
      transition: all 0.1s ease-in-out;

    }
    h3:hover{
        background-color: var(--green);
        color: var(--surface0);
        scale : 1.1;
    }
    h3:active{
        background-color: var(--teal);
        scale : 1;
    }
    h1{
      font-size: 3rem;
      margin-bottom: 0.5em;
      font-weight: 900;
    }
    .resp{
      font-size: 1.5rem;
      margin-top: 1em;
      font-weight: 500;
      margin: 1em 0 0 0;
    }

</style>
