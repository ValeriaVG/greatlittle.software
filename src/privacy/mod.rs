use std::fs;
use std::path::Path;

use macros::html_template;
use pulldown_cmark::{html as cmark_html, Options, Parser};
use serde::Deserialize;

use crate::html::{finalize, template, Bundle};
use crate::theme::{layout, SITE_URL};

html_template!(privacy, "src/privacy");

const TITLE: &str = "Privacy Policy";
const DESCRIPTION: &str = "Privacy policy for Great Little Software.";

#[derive(Default, Deserialize)]
#[serde(default)]
struct PrivacyFrontMatter {
    title: String,
    description: String,
}

pub fn build(content_root: &Path, out_root: &Path) -> std::io::Result<String> {
    let bundle = render(content_root);
    let page = finalize(bundle);
    let out_dir = out_root.join("privacy");
    fs::create_dir_all(&out_dir)?;
    let out = out_dir.join("index.html");
    fs::write(&out, &page)?;
    Ok(out.display().to_string())
}

fn render(content_root: &Path) -> Bundle {
    let index_md = content_root.join("privacy").join("index.md");
    let raw = fs::read_to_string(&index_md).unwrap_or_default();
    let (fm_yaml, body_md) = split_frontmatter(&raw);
    let _fm: PrivacyFrontMatter = serde_yaml::from_str(fm_yaml)
        .unwrap_or_else(|e| panic!("invalid frontmatter in content/privacy/index.md: {e}"));

    let body_html = render_markdown(body_md.trim());
    let body_bundle = Bundle { html: body_html, css: String::new(), js: String::new() };

    let title = if _fm.title.is_empty() { TITLE } else { &_fm.title };
    let description = if _fm.description.is_empty() { DESCRIPTION } else { &_fm.description };

    let page = privacy(body_bundle);
    let canonical = format!("{SITE_URL}/privacy/");
    layout(title, description, &canonical, page)
}

fn render_markdown(md: &str) -> String {
    if md.is_empty() {
        return String::new();
    }
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_SMART_PUNCTUATION);
    let parser = Parser::new_ext(md, opts);
    let mut out = String::new();
    cmark_html::push_html(&mut out, parser);
    out
}

fn split_frontmatter(raw: &str) -> (&str, &str) {
    let rest = raw
        .strip_prefix("---\n")
        .or_else(|| raw.strip_prefix("---\r\n"))
        .unwrap_or_else(|| panic!("missing frontmatter start `---` in content/privacy/index.md"));
    let end = rest
        .find("\n---")
        .unwrap_or_else(|| panic!("missing frontmatter end `---` in content/privacy/index.md"));
    let fm = &rest[..end];
    let after = &rest[end + 4..];
    let body = after
        .strip_prefix('\n')
        .or_else(|| after.strip_prefix("\r\n"))
        .unwrap_or(after);
    (fm, body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_frontmatter() {
        let raw = "---\ntitle: Hi\n---\nBody\n";
        let (fm, body) = split_frontmatter(raw);
        assert_eq!(fm, "title: Hi");
        assert_eq!(body, "Body\n");
    }
}