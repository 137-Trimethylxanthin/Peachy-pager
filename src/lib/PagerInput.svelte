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
      pageResponse = 'Connecting... (kann bis zu 10 sekunden dauern pro versuch haben 3 versuche)'
      //sleep for like 01 secs
      await new Promise(r => setTimeout(r, 500));
    try {
      pageResponse = await invoke('connect', { pageNum });
      // Handle the successful response
      console.log(pageResponse);
    } catch (error) {
      if (error === 'Failed to connect after 3 retries') {
        console.log('error');
        const maxRetries = 3;
        const errorMessage = `Failed to connect after ${maxRetries} retries`;
        pageResponse = errorMessage;
        // Handle the error message
    } else {
      // Handle other errors
      pageResponse = error +" -_- didnt work idk why";

  }
}


    }
  }

  function validateInput() {
    pageNum = pageNum.replace(/[^0-9]/g, '');
  }
</script>

<div>
  <input type="text" id="greet-input" placeholder="123" maxlength="3" pattern="[0-9]*" bind:value="{pageNum}" on:input={validateInput}  />
  <button class="main-buttons" on:click="{send}">connect</button>
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