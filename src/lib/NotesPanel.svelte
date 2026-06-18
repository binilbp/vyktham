<script>
    import { FileClock, ListTree } from '@lucide/svelte'

    const viewModes = Object.freeze({
        CHRON: 'chron',  //chronological
        HIERA: 'hiera',  //heirarchial
    });

    let viewMode = $state(viewModes.CHRON);


</script>
<div class = "bg-panel-base w-80 h-full flex flex-col justify-between">

    {#if viewMode === viewModes.CHRON }
    <section class="flex flex-col items-start justify-start gap-2 p-5 border-1">
        <span> note5 </span>
        <span> note4 </span>
        <span> note3 </span>
        <span> note2 </span>
        <span> note1 </span>
        
    </section>
    {:else}
    <section class="flex flex-col items-start justify-start gap-2 p-5 border-1">
        <span> note3 </span>
        <span> note1 </span>
        <span> note2 </span>
        <span> note5 </span>
        <span> note4 </span>
        
    </section>
    {/if}


    <section class="flex flex-row items-center justify-around">
        <button 
        onclick={()=>{viewMode = viewModes.CHRON}}
        class="w-full py-2 text-txt-ui flex justify-center item-center  {viewMode === viewModes.CHRON ? "bg-panel-active" : "bg-panel-base hover:bg-panel-hover"}"
        >
            <FileClock size={20}/>
        </button>

        <button
        onclick={()=>{viewMode = viewModes.HIERA}}
        class="w-full py-2 text-txt-ui flex justify-center item-center {viewMode === viewModes.HIERA ? "bg-panel-active" : "bg-panel-base hover:bg-panel-hover "}"
        >
            <ListTree size={20}/>
        </button>
    </section>

</div>
