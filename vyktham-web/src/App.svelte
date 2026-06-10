<script>
    import { appSettings } from './settings.svelte.js'
    
    import Clock from './lib/Clock.svelte'
    import MainBoard from './lib/MainBoard.svelte'

    //icons
    // import { Sun, Cog } from '@lucide/svelte/icons/sun'
    import { Sun, Cog } from '@lucide/svelte'
    // import { Cog } from '@lucide-svelte'
    import Github from './assets/Github.svelte'


</script>

<div class="flex flex-col justify-between h-dvh {appSettings.colors.background}">
    <header class="px-10 py-4 flex flex-row justify-between items-center ">
        <h1 class="font-cascadia text-2xl {appSettings.colors.heading}">
            Vyktham
        </h1>     

        <nav class="flex flex-row justify-between items-center gap-6">
            <div class="{appSettings.colors.muted}"> <Sun size={19}/> </div> 
            <div class="{appSettings.colors.muted}"> <Cog size={19}/> </div> 
        </nav>
    </header>

    <main class="px-34 flex-grow min-h-0">
        <MainBoard />
    </main>

    <footer class="px-10 py-4 flex flex-row justify-between ">
        <section class="flex flex-row justify-between gap-10">
            <h3 class="font-fragment text-sm {appSettings.colors.muted}"> About </h3> 
            
            <a href="https://github.com/binilbp/vyktham">
                <Github class="size-5 {appSettings.colors.muted} {appSettings.colors.hover}" />
            </a>
        </section>
        <section>
            <Clock />
        </section>
    </footer>
</div>

