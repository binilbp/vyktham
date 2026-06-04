<script>
    import {appSettings} from "./settings.svelte.ts"
</script>

<div class="flex flex-col justify-between px-46 py-5 h-dvh {appSettings.colors.background}">
    <header class="flex flex-row justify-between items-center">
        <h1 class="font-cascadia text-3xl {appSettings.colors.muted}">
            Vyktham
        </h1>     

        <nav class="flex flex-row justify-center gap-4">
            <h3 class="{appSettings.colors.muted}"> vim </h3> 
            <h3 class="{appSettings.colors.muted}"> theme </h3> 
        </nav>
    </header>

    <main>
        <p> main part goes here </p>
    </main>

    <footer class="flex flex-row justify-between">
        <section class="flex flex-row justify-center gap-4">
            <h3 class="{appSettings.colors.muted}"> About </h3> 
            <h3 class="{appSettings.colors.muted}">  </h3> 
        </section>
        <section>
            <h3> 10:40 Thursday </h3>
        </section>


    </footer>
</div>

