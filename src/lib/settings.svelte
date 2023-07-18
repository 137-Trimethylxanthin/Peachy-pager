<script>
  import { invoke } from '@tauri-apps/api/tauri'

let path = ""

  async function first_time_file(){
    path = await invoke('first_time_file')
  }
</script>

<div>
    <button class="main-buttons" on:click="{first_time_file}">test config file</button>
    <p>{path}</p>
</div>

<style>
  div{
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
  }
  button{
    width: 8em;
    height: 100%;
    font-size: 2rem;
    text-align: center;
    margin-top: 0.5em;
    background-color: var(--surface0);
    border: 1px solid var(--surface1);
    border-radius: 0.5em;
    box-shadow: 0 0 0 0.1em var(--surface1) inset;
  }
  button:hover{
    background-color: var(--green);
    color: var(--surface0);
  }
  p{
    color: var(--subtext0);
    text-align: center;
  }
</style>