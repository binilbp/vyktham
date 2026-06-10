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
                    class: `prose prose-slate h-full bg-white overflow-y-auto p-20 focus:outline-none ${appSettings.font}`,
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

<style>
    :global(.tiptap p.is-editor-empty:first-child::before ){
      color: #adb5bd;
      content: attr(data-placeholder);
      float: left;
      height: 0;
      pointer-events: none;
    }
</style>

<div 
    class = "h-full min-h-0 shadow-xl rounded-xl" 
    bind:this={element}>
</div>

