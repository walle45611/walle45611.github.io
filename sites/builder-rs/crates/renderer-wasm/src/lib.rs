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

fn comrak_options() -> ComrakOptions {
    ComrakOptions::default()
}
