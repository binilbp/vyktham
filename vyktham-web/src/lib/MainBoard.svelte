<script>

import InputPanel from './InputPanel.svelte'

async function handleSubmit(userText) {
    if (!userText.trim()) return;

    try {
        const response = await fetch('localhost:8000/usertext', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({userText})
        });
        console.log('INFO: user text sent');

        const data = await response.json();
        console.log('DEBUG: output recieved:', data);

    } catch (err) {
        console.log('ERROR: user text not sent', err);
    }
}

</script>

<div class = "py-30 flex flex-col justify-between items-center w-full ">
    <InputPanel {handleSubmit} />
</div>

