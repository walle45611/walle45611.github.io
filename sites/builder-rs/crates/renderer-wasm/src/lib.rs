use comrak::{markdown_to_html, ComrakOptions};

pub fn render_markdown_to_html(markdown: &str) -> String {
    markdown_to_html(markdown, &comrak_options())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn render_markdown(markdown: &str) -> String {
    render_markdown_to_html(markdown)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn render_markdown(markdown: &str) -> String {
    render_markdown_to_html(markdown)
}

fn comrak_options() -> ComrakOptions<'static> {
    let mut options = ComrakOptions::default();
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.tasklist = true;
    options.extension.autolink = true;
    options.extension.superscript = true;
    options.extension.header_ids = Some("".to_string());
    options.parse.smart = true;
    options.render.unsafe_ = true;
    options.render.github_pre_lang = true;
    options.extension.footnotes = true;
    options
}
