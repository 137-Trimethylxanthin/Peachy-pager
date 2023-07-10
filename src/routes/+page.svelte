<script>
	import PagerInput from "../lib/PagerInput.svelte";
	import Settings from "../lib/settings.svelte";

</script>
<body>
    <h1>Pager Nummer eingeben:</h1>

    <PagerInput />
    <br>
    <Settings />
</body>


<style>
    body{
        display: flex;
        flex-direction: column;
        max-width: 800px;
        margin: 0 auto;
        align-items: center;
    }

</style>