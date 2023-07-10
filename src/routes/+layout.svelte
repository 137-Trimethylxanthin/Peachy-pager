<script>
	import { _toggleDarkMode} from "./+layout";
    import "./styles.css"

    let icon = ""
    let theme = ""
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
        console.log("settings")
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

<main>
    <slot></slot>
</main>

<footer>

</footer>


<style>

</style>