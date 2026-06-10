<script>
    import { appSettings } from '../settings.svelte.js'
</script>

<div class ="flex flex-row justify-center gap-3">
    <button class ="px-2 py-1 rounded-xl  bg-slate-700 text-white {appSettings.font} ">
        Edit
    </button>

    <button class ="px-2 py-1 rounded-xl border-2 border-slate-700 {appSettings.font} ">
        View
    </button>

</div>
