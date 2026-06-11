<script>
    let { toggleEditorLock, editorLock } = $props();
    import { appSettings } from '../settings.svelte.js'
    import { LockKeyholeOpen } from '@lucide/svelte'
    import { LockKeyhole } from '@lucide/svelte'

</script>

<div class ="flex flex-col justify-start gap-3">

    <button 
    class ="{appSettings.colors.muted} rounded-xl shadow-md p-2 hover:text-slate-500 bg-white hover:bg-slate-100"
    onclick = {toggleEditorLock}
    >
        {#if editorLock}
            <LockKeyhole size = {20} />     
        {:else}
            <LockKeyholeOpen size = {20} />     
        {/if}
    </button>

</div>
