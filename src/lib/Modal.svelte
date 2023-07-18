<script lang="ts">
    let is_open = false;
    import { fade } from 'svelte/transition';
    import { is_open as isOpen} from './store';
    $: is_open = $isOpen;
    function close() {
        console.log("modal",is_open)
    }
</script>

{#if is_open}
    <div class="modal" transition:fade>
        <div class="modal-content">
            <slot></slot>
            <button on:click={close}></button>
        </div>
    </div>
{/if}

<style>
    .modal {
        position: fixed;
        z-index: 1;
        left: 0;
        top: 0;
        width: 100%;
        height: 100%;
        display: flex;
        justify-content: center;
        align-items: center;
        background-color: rgba(0, 0, 0, 0.5); /* Background color with transparency */
        transition: opacity 0.3s;
        z-index: 9999;
    }
    @keyframes fade {
    from {
        opacity: 0;
    }

    to {
        opacity: 1;
    }
    }

    .modal-content {
        background-color: var(--crust);
        margin: auto;
        padding: 20px;
        border: 1px solid var(--teal);
        width: 80%;
        display: flex;
        flex-direction: column;
        align-items: center;
        z-index: 10000;
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