<script>
  import { invoke } from '@tauri-apps/api/tauri'

  let pageNum = ''
  let pageResponse = ''

  async function send() {
    if (pageNum === '') {
      pageResponse = 'Keine Nummer -_-'
      return
      
    } else if (pageNum.length !== 3) {
      pageResponse = 'Bitte eine 3 stellige number'
      return

    } else{
      pageResponse = await invoke('sendNewPage', { pageNum })
    }
  }

  function validateInput() {
    pageNum = pageNum.replace(/[^0-9]/g, '');
  }
</script>

<div>
  <input type="text" id="greet-input" placeholder="123" maxlength="3" pattern="[0-9]*" bind:value="{pageNum}" on:input={validateInput}  />
  <button on:click="{send}">connect</button>
  <p>{pageResponse}</p>
</div>

<style>
  div{
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
  }
  input{
    width: 100%;
    height: 100%;
    font-size: 2rem;
    text-align: center;
    margin-top: 0.5em;
    background-color: var(--surface0);
    border: 1px solid var(--surface1);
    border-radius: 0.5em;
    box-shadow: 0 0 0.1em 0.1em var(--surface1) inset;
  }
  input:hover{
    box-shadow: 0 0 10px var(--surface2);
  }
  button{
    width: 60%;
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