use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::process::{Command, Stdio};

use crate::html::Bundle;

const CSS_PATH: &str = "src/blog/mermaid/mermaid.css";

// Rendered diagrams are cached here, keyed by the mermaid source hash, so the
// build stays offline and deterministic once a diagram has been rendered. The
// cache is meant to be committed to the repository.
const CACHE_DIR: &str = ".mermaid-cache";
const RENDER_URL: &str = "https://kroki.io/mermaid/svg";
// Bump when the recoloring in `theme_svg` changes so stale cached SVGs are ignored.
const THEME_VERSION: &str = "v1";

const OPEN: &str = r#"<pre><code class="language-mermaid">"#;
const CLOSE: &str = "</code></pre>";

pub fn css() -> String {
    std::fs::read_to_string(CSS_PATH).unwrap_or_default()
}

pub fn has_mermaid(html: &str) -> bool {
    html.contains(OPEN)
}

/// Replaces mermaid fenced code blocks with the rendered SVG, inlined inside a
/// `<div class="mermaid-svg">` figure. The inner code-block content is HTML that
/// pulldown-cmark escaped, so it is unescaped back to the original mermaid source
/// before rendering. If rendering is unavailable (offline, tool missing) the
/// original code block is left in place so the article still builds.
///
/// The visual SVG is hidden from assistive tech (`aria-hidden`) and paired with
/// a visually-hidden text alternative derived from the mermaid source, since
/// the source is itself a textual definition of the diagram.
pub fn transform(html: &str) -> String {
    if !has_mermaid(html) {
        return html.to_string();
    }
    let mut out = String::with_capacity(html.len());
    let mut rest = html;
    while let Some(start) = rest.find(OPEN) {
        out.push_str(&rest[..start]);
        let after = &rest[start + OPEN.len()..];
        match after.find(CLOSE) {
            Some(end) => {
                let source = unescape_html(&after[..end]);
                let block_len = OPEN.len() + end + CLOSE.len();
                match render_to_svg(&source) {
                    Some(svg) => {
                        out.push_str("<div class=\"mermaid-svg\">");
                        out.push_str(&hide_from_at(&svg));
                        out.push_str("<p class=\"mermaid-alt\">Diagram (text alternative):</p>");
                        out.push_str("<pre class=\"mermaid-alt\">");
                        out.push_str(&crate::html::escape_html(&text_alt(&source)));
                        out.push_str("</pre>");
                        out.push_str("</div>");
                    }
                    None => out.push_str(&rest[start..start + block_len]),
                }
                rest = &after[end + CLOSE.len()..];
            }
            None => {
                out.push_str(&rest[start..]);
                return out;
            }
        }
    }
    out.push_str(rest);
    out
}

/// Transforms mermaid code blocks in `body_html` to inlined SVG and returns a
/// Bundle carrying the figure styles only when a diagram is present.
pub fn bundle(body_html: &str) -> Bundle {
    let html = transform(body_html);
    if has_mermaid(body_html) {
        Bundle {
            html,
            css: css(),
            js: String::new(),
        }
    } else {
        Bundle {
            html,
            css: String::new(),
            js: String::new(),
        }
    }
}

fn unescape_html(s: &str) -> String {
    s.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&amp;", "&")
}

/// Marks the root `<svg>` as hidden from assistive tech so VoiceOver does not read
/// the diagram's internal labels and edges as disconnected fragments. The text
/// alternative next to it carries the meaning.
fn hide_from_at(svg: &str) -> String {
    svg.replacen("<svg", "<svg aria-hidden=\"true\"", 1)
}

/// Builds a screen-reader text alternative from the mermaid source itself.
/// Drops the diagram-type directive (`flowchart`/`graph`) and turns `<br/>`
/// line breaks inside labels into spaces, leaving the edge definitions readable
/// line by line.
fn text_alt(source: &str) -> String {
    let mut out = String::with_capacity(source.len());
    for line in source.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("flowchart") || trimmed.starts_with("graph") {
            continue;
        }
        if !out.is_empty() {
            out.push('\n');
        }
        out.push_str(line);
    }
    out.replace("<br/>", " ").replace("<br>", " ")
}

/// Renders mermaid `source` to SVG, returning a cached result when available.
/// Returns `None` on any failure so the caller can degrade to the code block
/// instead of failing the build.
fn render_to_svg(source: &str) -> Option<String> {
    render_to_svg_with(source, RENDER_URL)
}

fn render_to_svg_with(source: &str, url: &str) -> Option<String> {
    let hash = hash_hex(source);
    let cache_path = format!("{CACHE_DIR}/{THEME_VERSION}-{hash}.svg");
    if let Ok(cached) = std::fs::read_to_string(&cache_path) {
        if cached.trim_start().starts_with("<svg") {
            return Some(cached);
        }
    }
    std::fs::create_dir_all(CACHE_DIR).ok()?;
    let mut cmd = Command::new("curl");
    cmd.args([
        "-sS",
        "--fail",
        "--max-time",
        "90",
        "-X",
        "POST",
        "-H",
        "Content-Type: text/plain",
        "--data-binary",
        "@-",
        "-o",
        &cache_path,
        url,
    ]);
    cmd.stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    let mut child = cmd.spawn().ok()?;
    {
        let mut stdin = child.stdin.take()?;
        stdin.write_all(source.as_bytes()).ok();
    }
    let status = child.wait().ok()?;
    if !status.success() {
        let _ = std::fs::remove_file(&cache_path);
        return None;
    }
    let svg = std::fs::read_to_string(&cache_path).ok()?;
    if !svg.trim_start().starts_with("<svg") {
        let _ = std::fs::remove_file(&cache_path);
        return None;
    }
    // kroki writes the raw mermaid output; rewrite it on disk in-place so the
    // cache holds the theme-aware version.
    let themed = theme_svg(&svg);
    if std::fs::write(&cache_path, &themed).is_err() {
        return Some(svg);
    }
    Some(themed)
}

fn hash_hex(s: &str) -> String {
    let mut h = DefaultHasher::new();
    s.hash(&mut h);
    format!("{:016x}", h.finish())
}

/// Rewrites mermaid's hardcoded palette tokens to the site's theme variables so
/// the inlined diagram inherits the current dark/light theme via CSS cascade.
/// Longer hex tokens are replaced before their shorter prefixes.
fn theme_svg(svg: &str) -> String {
    svg.replace("#333333", "var(--color-ink)")
        .replace("#333", "var(--color-ink)")
        .replace("#000000", "var(--color-ink)")
        .replace("#000", "var(--color-ink)")
        .replace("#9370DB", "var(--color-ink)")
        .replace("#ECECFF", "var(--color-surface-warm)")
        .replace("rgba(232,232,232, 0.8)", "var(--color-bg)")
        .replace("rgba(232, 232, 232, 0.5)", "var(--color-bg)")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_mermaid_detects_only_mermaid() {
        assert!(has_mermaid(
            "<pre><code class=\"language-mermaid\">x</code></pre>"
        ));
        assert!(!has_mermaid(
            "<pre><code class=\"language-rust\">x</code></pre>"
        ));
    }

    #[test]
    fn unescape_html_reverses_code_block_escaping() {
        assert_eq!(
            unescape_html("a --&gt; b &amp; c &lt;br/&gt; d"),
            "a --> b & c <br/> d"
        );
        assert_eq!(unescape_html("&quot;q&quot; &#39;s"), "\"q\" 's");
    }

    #[test]
    fn transform_preserves_other_code_blocks() {
        let html = "<pre><code class=\"language-rust\">fn main() {}</code></pre>";
        assert_eq!(transform(html), html);
    }

    #[test]
    fn render_returns_none_when_endpoint_unreachable() {
        // An unroutable endpoint exercises the failure path without network:
        // a None result is what lets `transform` fall back to the code block.
        let out = render_to_svg_with("flowchart LR\n    A --> B\n", "http://127.0.0.1:1/nope");
        assert!(out.is_none());
    }

    #[test]
    fn theme_svg_swaps_palette_for_css_variables() {
        let svg = "<style>.x{fill:#ECECFF;stroke:#9370DB}.y{fill:#333;color:#333333}z{fill:#000;stroke:#000000}</style></svg>";
        let out = theme_svg(svg);
        assert!(!out.contains("#333"), "ink hexes should be gone: {out}");
        assert!(!out.contains("#9370DB"));
        assert!(!out.contains("#ECECFF"));
        assert!(out.contains("var(--color-ink)"));
        assert!(out.contains("var(--color-surface-warm)"));
    }

    #[test]
    fn hide_from_at_marks_root_svg() {
        let out = hide_from_at("<svg class=x></svg></svg>");
        assert!(out.starts_with("<svg aria-hidden=\"true\" class=x>"));
    }

    #[test]
    fn text_alt_drops_directive_and_unwraps_br() {
        let src = "flowchart LR\n    A[\"Idea\"] --> B{\"Two<br/>lines\"}\n";
        let out = text_alt(src);
        assert!(!out.contains("flowchart"));
        assert!(!out.contains("<br"));
        assert!(out.contains("Two lines"));
        assert!(out.contains("-->"));
    }
}

#[cfg(feature = "preview")]
mod previews {
    use macros::preview;

    use crate::html::Bundle;

    use super::bundle;

    const SAMPLE: &str = concat!(
        "<p>An article with a diagram in the prose.</p>\n",
        "<pre><code class=\"language-mermaid\">flowchart LR\n",
        "    Idea[\"Idea or Pivot\"] --&gt; Exp[\"Experimentation\"]\n",
        "    Exp --&gt; Idea\n",
        "</code></pre>\n",
    );

    #[preview("Mermaid/Flowchart")]
    fn flowchart() -> Bundle {
        bundle(SAMPLE)
    }
}
