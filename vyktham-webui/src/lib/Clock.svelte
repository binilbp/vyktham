<script lang="ts"> 
    import { appSettings } from '../settings.svelte.ts'
    import { onMount } from 'svelte';

    const formatter = new Intl.DateTimeFormat([], {
        hour: '2-digit',
        minute: '2-digit'
    });

    let formattedTime: string = formatter.format(new Date());

    onMount(() => {
        let timeout: ReturnType<typeof setTimeout>;

        const tick = () => {
          const now = new Date();
          
          formattedTime = formatter.format(now);

          const seconds = now.getSeconds();
          const milliseconds = now.getMilliseconds();
          const msUntilNextMinute = 60000 - (seconds * 1000 + milliseconds);

          timeout = setTimeout(tick, msUntilNextMinute);
        };

        tick();

        return () => clearTimeout(timeout);
    });

</script>

<span class="font-fragment text-sm {appSettings.colors.muted}">{formattedTime}</span>

