<script lang="ts">
	import { _toggleDarkMode} from "./+layout";
    import "./styles.css"
    import Modal from '../lib/Modal.svelte';
    import Options from '../lib/Options.svelte';
    import {is_open} from "../lib/store"

    let icon = ""
    let theme = ""
    if (document.cookie.length == 0){
        document.cookie = "setTheme=light;max-age=31536000;path=/"
        document.cookie = "cMode=Easy;max-age=31536000;path=/"
    }


    console.log(document.documentElement.dataset.theme)
    if (document.documentElement.dataset.theme === "dark"){
        icon = "sun"
        theme = "dark"
    } else {
        icon = "moon"
        theme = "light"
    }

    let state = "no-smol"

    function click(){
        document.documentElement.dataset.theme = document.documentElement.dataset.theme === "dark" ? "light" : "dark";
        document.cookie = `setTheme=${document.documentElement.dataset.theme};max-age=31536000;path=/`;
        if (document.documentElement.dataset.theme === "dark"){
            icon = "sun"
            theme = "dark"
        } else {
            icon = "moon"
            theme = "light"
        }
    }

    function settingsPopup(){
        is_open.set(true);
    }


</script>

<header>
    <ul>
        <li><h2>Peachy-pager</h2></li>
        

        <li><img class="icons" src="./media/{icon}.png" alt="" on:click={click}></li>
        <li><img class="icons" src="./media/info-{theme}.png" alt=""></li>
        {#if state === "smol"}
        <li><img class="icons" src="./media/window-make-big-{theme}.png" alt=""></li>
        {:else}
        <li><img class="icons" src="./media/window-make-smol-{theme}.png" alt=""></li>
        {/if}
        <li><img class="icons" src="./media/option-{theme}.png" alt="" on:click={settingsPopup}></li>
    </ul>
</header>
<Modal>
    <Options />
</Modal>

<main>
    <slot></slot>
</main>

<footer>

</footer>


