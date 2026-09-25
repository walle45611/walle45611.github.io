use comrak::{markdown_to_html, ComrakOptions};

pub fn render_markdown_to_html(markdown: &str) -> String {
    let protected = protect_display_math(markdown);
    let html = markdown_to_html(&protected, &comrak_options());
    restore_math_delimiters(&html)
}

fn protect_display_math(markdown: &str) -> String {
    let mut out = String::with_capacity(markdown.len());
    let mut lines = markdown.lines().peekable();
    let mut fence = false;
    while let Some(line) = lines.next() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            fence = !fence;
        }
        if !fence && line.trim() == "$$" {
            let mut math = Vec::new();
            let mut closed = false;
            while let Some(next) = lines.peek() {
                if next.trim() == "$$" {
                    lines.next();
                    closed = true;
                    break;
                }
                math.push(lines.next().unwrap_or_default());
            }
            if !closed {
                out.push_str("$$\n");
                for math_line in math {
                    out.push_str(math_line);
                    out.push('\n');
                }
                break;
            }
            out.push_str("\n<div class=\"math-tex math-display\">\\[\n");
            for math_line in math {
                out.push_str(
                    &math_line
                        .replace('&', "&amp;")
                        .replace('<', "&lt;")
                        .replace('>', "&gt;"),
                );
                out.push('\n');
            }
            out.push_str("\\]</div>\n\n");
        } else {
            out.push_str(line);
            out.push('\n');
        }
    }
    out
}

fn restore_math_delimiters(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut rest = html;
    while let Some(start) = rest.find("<span data-math-style=\"") {
        out.push_str(&rest[..start]);
        rest = &rest[start..];
        let (opening, left, right) = if rest.starts_with("<span data-math-style=\"display\">") {
            ("<span data-math-style=\"display\">", r"\[", r"\]")
        } else if rest.starts_with("<span data-math-style=\"inline\">") {
            ("<span data-math-style=\"inline\">", r"\(", r"\)")
        } else {
            const PREFIX: &str = "<span data-math-style=\"";
            out.push_str(PREFIX);
            rest = &rest[PREFIX.len()..];
            continue;
        };
        if let Some(end) = rest[opening.len()..].find("</span>") {
            let content = &rest[opening.len()..opening.len() + end];
            out.push_str("<span class=\"math-tex\">");
            out.push_str(left);
            out.push_str(content);
            out.push_str(right);
            out.push_str("</span>");
            rest = &rest[opening.len() + end + "</span>".len()..];
        } else {
            out.push_str(rest);
            return out;
        }
    }
    out.push_str(rest);
    out
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
    options.extension.math_dollars = true;
    options
}

#[cfg(test)]
mod tests {
    use super::render_markdown_to_html;

    #[test]
    fn preserves_latex_and_display_math() {
        let html = render_markdown_to_html(
            "Inline $O(n^2)$\n\n$$\n\\begin{bmatrix}\na&b\\\\\nc&d\n\\end{bmatrix}\n$$",
        );
        assert!(html.contains(r"\(O(n^2)\)"));
        assert!(html.contains(r"\["));
        assert!(html.contains(r"b\\"));
        assert!(!html.contains("data-math-style"));
        assert!(html.contains("<div class=\"math-tex math-display\">"));
    }

    #[test]
    fn protects_multiline_math_with_subscripts() {
        let html = render_markdown_to_html(
            "$$\n\\begin{aligned}\n&= \\sum_{i=1}^{n} \\operatorname{depth}_T(k_i) \\\\\n\\end{aligned}\n$$",
        );
        assert!(html.contains("\\sum_{i=1}^{n}"));
        assert!(html.contains("\\operatorname{depth}_T(k_i)"));
        assert!(!html.contains("<sup>"));
    }
}
