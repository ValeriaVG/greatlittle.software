use std::path::Path;

use macros::html_template;
use pulldown_cmark::{html as cmark_html, Options, Parser};
use serde::Deserialize;

use crate::html::{escape_html, template, Bundle};

html_template!(faq, "src/home/faq");

#[derive(Default, Deserialize)]
struct FrontMatterItem {
    q: String,
    a: String,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct FaqFrontMatter {
    faq_title: String,
    faq: Vec<FrontMatterItem>,
}

pub fn load(content_root: &Path) -> Option<Bundle> {
    let index_md = content_root.join("index.md");
    if !index_md.exists() {
        return None;
    }
    let raw = std::fs::read_to_string(&index_md).ok()?;
    let (fm_yaml, body_md) = split_frontmatter(&raw);
    let fm: FaqFrontMatter = serde_yaml::from_str(fm_yaml)
        .unwrap_or_else(|e| panic!("invalid frontmatter in content/index.md: {e}"));

    let mut items_html = String::new();
    for item in &fm.faq {
        items_html.push_str("<dt class=\"home-faq-q\">");
        items_html.push_str(&escape_html(&item.q));
        items_html.push_str("</dt>\n<dd class=\"home-faq-a\">");
        items_html.push_str(&render_markdown(&item.a));
        items_html.push_str("</dd>\n");
    }

    let body_html = render_markdown(body_md.trim());
    let has_body = if body_html.is_empty() { "" } else { "yes" };

    let items_bundle = Bundle { html: items_html, css: String::new(), js: String::new() };
    let body_bundle = Bundle { html: body_html, css: String::new(), js: String::new() };

    let items: Vec<FaqItem> = fm.faq.iter().map(|item| FaqItem {
        q: item.q.clone(),
        a: item.a.clone(),
    }).collect();
    let faq_items: &[FaqItem] = &items;

    let bundle = faq(
        &fm.faq_title,
        items_bundle,
        has_body,
        body_bundle,
        faq_items,
    );

    Some(bundle)
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
        .unwrap_or_else(|| panic!("missing frontmatter start `---` in content/index.md"));
    let end = rest.find("\n---").unwrap_or_else(|| panic!("missing frontmatter end `---` in content/index.md"));
    let fm = &rest[..end];
    let after = &rest[end + 4..];
    let body = after
        .strip_prefix('\n')
        .or_else(|| after.strip_prefix("\r\n"))
        .unwrap_or(after);
    (fm, body)
}

#[cfg(feature = "preview")]
mod previews {
    use macros::preview;

    use crate::html::Bundle;

    use super::{faq, FaqItem};

    #[preview("Home/FAQ")]
    fn default() -> Bundle {
        let items_html = "\
<dt class=\"home-faq-q\">What is great little software?</dt>\n\
<dd class=\"home-faq-a\"><p>Independent apps built by passionate people, not corporations.</p></dd>\n\
<dt class=\"home-faq-q\">Who is behind this?</dt>\n\
<dd class=\"home-faq-a\"><p>People like you.</p></dd>".to_string();
        let items = Bundle { html: items_html, css: String::new(), js: String::new() };
        let faq_items = vec![
            FaqItem { q: "What is great little software?".into(), a: "Independent apps built by passionate people, not corporations.".into() },
            FaqItem { q: "Who is behind this?".into(), a: "People like you.".into() },
        ];
        faq("FAQ", items, "", Bundle { html: String::new(), css: String::new(), js: String::new() }, &faq_items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_frontmatter() {
        let raw = "---\nfaq_title: Hi\n---\nBody\n";
        let (fm, body) = split_frontmatter(raw);
        assert_eq!(fm, "faq_title: Hi");
        assert_eq!(body, "Body\n");
    }

    #[test]
    fn parses_faq_items() {
        let y = "faq:\n  - q: \"What?\"\n    a: \"That.\"\n";
        let fm: FaqFrontMatter = serde_yaml::from_str(y).unwrap();
        assert_eq!(fm.faq.len(), 1);
        assert_eq!(fm.faq[0].q, "What?");
        assert_eq!(fm.faq[0].a, "That.");
    }

    #[test]
    fn empty_frontmatter_defaults() {
        let y = "";
        let fm: FaqFrontMatter = serde_yaml::from_str(y).unwrap();
        assert!(fm.faq.is_empty());
        assert_eq!(fm.faq_title, "");
    }
}