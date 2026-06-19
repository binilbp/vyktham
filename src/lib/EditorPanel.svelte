<script>
    import { invoke } from '@tauri-apps/api/core';
    import { onMount, onDestroy } from 'svelte';

    import { Editor } from '@tiptap/core';
    import { StarterKit } from '@tiptap/starter-kit';
    import { Placeholder } from '@tiptap/extensions';
    import { Markdown } from '@tiptap/markdown'
    async function handleSubmit(tiptapMarkdown) {
        try {
            console.log('INFO: sending tiptapMarkdown');
            invoke('format_markdown', { text: tiptapMarkdown }).then((response) => {
                console.log('INFO: output recieved');
                editorState.editor.commands.setContent(response, { contentType: 'markdown' } );
                console.log('INFO: updated to ai markdown format');
                
            });
        } catch (err) {
            console.log('ERROR: tiptapjson not sent', err);
        }
    }

    let element = $state();
    let editorState = $state({editor: null});


    onMount(() => {
        editorState.editor = new Editor({
            element: element,
            extensions: [
                StarterKit,
                Placeholder.configure({
                    placeholder: 'Paste your mind here !',
                }),
                Markdown,
            ],
            contentType: 'markdown',
            editorProps: {
                attributes: {
                    class: "pt-20 mx-auto max-w-none w-[80%] h-full break-words prose prose-slate prose-invert text-txt-editor text-lg font-jetbrains focus:outline-none",
                },
                handleKeyDown: (view, event) => {
                    if (event.key === 'Enter' && (event.ctrlKey || event.metaKey)) {
                        const ttMarkdown = editorState.editor.getMarkdown();
                        handleSubmit(ttMarkdown);
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

<div class=" flex-1 min-w-0 min-h-0 bg-editor-base flex flex-col justify-start items-center">
    <main 
        class=" h-full w-full overflow-y-auto" 
        bind:this={element}>
    </main>
</div>
