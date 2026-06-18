<script>
    import { onMount, onDestroy } from 'svelte';

    import { Editor } from '@tiptap/core';
    import { StarterKit } from '@tiptap/starter-kit';
    import { Placeholder } from '@tiptap/extensions';
    import { Markdown } from '@tiptap/markdown'

    async function handleSubmit(tiptapMarkdown) {
        try {
            console.log('INFO: sending tiptapMarkdown');
            console.log('INFO: output recieved');
            // console.log('DEBUG: recieved makdown is :', data.markdown);

            editorState.editor.commands.setContent('makdown set', { contentType: 'markdown' } );
            console.log('INFO: updated to ai markdown format');
            

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
                    class: "border-2 border-red-400 mx-auto max-w-none w-[80%] h-full break-words prose prose-slate prose-invert text-txt-editor text-xl font-jetbrains focus:outline-none",
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

<div class="border-amber-300 border-2 flex-1 min-w-0 min-h-0 bg-editor-base flex flex-col justify-start items-center">
    <main 
        class="border-blue-300 border-2 h-full w-full overflow-y-auto" 
        bind:this={element}>
    </main>
</div>
