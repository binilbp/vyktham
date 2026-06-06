<script>
    import { appSettings } from './settings.svelte.js'
    
    import Clock from './lib/Clock.svelte'

    //icons
    import Sun from '@lucide/svelte/icons/sun'
    import Github from './assets/Github.svelte'


</script>

<div class="flex flex-col justify-between px-44 py-5 h-dvh {appSettings.colors.background}">
    <header class="flex flex-row justify-between items-center">
        <h1 class="font-cascadia text-2xl {appSettings.colors.heading}">
            Vyktham
        </h1>     

        <nav class="flex flex-row justify-between items-center gap-6">
            <h3 class="font-fragment {appSettings.colors.muted}"> <Sun size={19}/> </h3> 
            <h3 class="font-fragment text-sm {appSettings.colors.muted}"> settings </h3> 
        </nav>
    </header>

    <main>
        <p class="text-center text-xl"> ""  </p>
    </main>

    <footer class="flex flex-row justify-between">
        <section class="flex flex-row justify-between gap-10">
            <h3 class="font-fragment text-sm {appSettings.colors.muted}"> About </h3> 
            <Github class="size-5 {appSettings.colors.muted}" />
        </section>
        <section>
            <Clock />
        </section>


    </footer>
</div>

