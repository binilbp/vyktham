<script>
let {handleSubmit} = $props();
import { appSettings } from '../settings.svelte.js';
import { onMount, onDestroy } from 'svelte';

import { Editor } from '@tiptap/core';
import { StarterKit } from '@tiptap/starter-kit';
import { Placeholder } from '@tiptap/extensions';


let element = $state();
let savedJSON ;
let editorState = $state({editor: null});

onMount(() => {
    editorState.editor = new Editor({
        element: element,
        extensions: [
            StarterKit,
            Placeholder.configure({
                placeholder: 'Paste your mind here !',
            }),
        ],
        editorProps: {
            attributes: {
                class: 'text-2xl h-full w-full overflow-y-auto p-20 focus:outline-none',
            },
            handleKeyDown: (view, event) => {
                if (event.key === 'Enter' && (event.ctrlKey || event.metaKey)) {
                    const userText = editorState.editor.getText(); 
                    handleSubmit(userText);
                    return true; //key event handled
                }
                return false; //key not handled
            },
        },
        autofocus: true,
        onTransaction: ({ editor }) => {
            // Update the state signal to force a re-render
            editorState = { editor }
        },
    })
})
onDestroy(() => {
    editorState.editor?.destroy()
})

</script>

<div 
    class="h-[50vh] w-full bg-white shadow-xl rounded-lg flex flex-col {appSettings.font}"
    bind:this={element}>
</div>

