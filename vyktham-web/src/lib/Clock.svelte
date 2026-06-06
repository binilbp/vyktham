<script> 
    import { appSettings } from '../settings.svelte.js'
    import { onMount } from 'svelte';

    const formatter = new Intl.DateTimeFormat([], {
        hour: '2-digit',
        minute: '2-digit'
    });

    // Wrapped in $state() so the UI updates every minute
    let formattedTime = $state(formatter.format(new Date()));

    onMount(() => {
        let timeout;

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
