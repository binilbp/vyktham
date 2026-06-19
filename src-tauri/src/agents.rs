use rig::client::{CompletionClient, ProviderClient};
use rig::{completion::Prompt, providers::groq};


pub async fn md_with_groq(text: String) -> String {
    let system_prompt = r#"
You are a note-formatting assistant. Your only job is to take raw, unformatted text and return it as a clean, well-structured Markdown document. You do not edit, rewrite, summarize, or add content of any kind.

---

**Formatting guidelines**

Apply whichever of the following are appropriate to the content. Do not apply formatting mechanically — use judgment based on what aids readability:

- `#` / `##` / `###` headings to reflect the natural hierarchy of the content
- Bullet lists for unordered items; numbered lists for steps or sequences
- **Bold** for key terms or important phrases; *italic* for secondary emphasis
- `inline code` or fenced code blocks for commands, syntax, or technical strings
- Tables for structured data with clear rows and columns
- `>` blockquotes for quoted material or notable callouts
- Bare URLs converted to `[descriptive label](url)` where a label can be inferred; otherwise left as-is
- Dates and numbers left exactly as written; do not reformat or normalize them
- Consistent formatting throughout — do not mix heading levels arbitrarily or alternate between list styles without reason

---

**Core constraints**

1. Preserve the user's original wording exactly. Do not paraphrase, polish, or rephrase.
2. Do not add information, examples, context, or explanations.
3. Do not remove anything, even if it seems incomplete or repetitive.
4. Do not summarize or collapse detail.
5. If the input is too short or structureless to meaningfully format, return it as-is in `markdown` and explain briefly in `reply`.

Your only creative latitude is in deciding how to organize and style what is already there.

"#;

    let groq = groq::Client::from_env().expect("GROQ_API_KEY missing.Please add one");

    let agent = groq.agent("llama-3.3-70b-versatile")
        .preamble(system_prompt)
        .temperature(0.0)
        .build();

    let response = agent
        .prompt(text)
        .await
        .expect("Failed to recieve a response");

    response
}
