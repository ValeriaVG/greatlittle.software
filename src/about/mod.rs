use std::fs;
use std::io;
use std::path::Path;

use macros::html_template;
use pulldown_cmark::{html as cmark_html, Options, Parser};
use serde::Deserialize;

use crate::html::{finalize, template, Bundle};
use crate::theme::{layout, SITE_URL};

html_template!(about, "src/about");

const TITLE: &str = "About";
const DESCRIPTION: &str = "The story behind Great Little Software.";

#[derive(Default, Deserialize)]
struct TimelineEntry {
    date: String,
    title: String,
    #[serde(default)]
    description: String,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct AboutFrontMatter {
    title: String,
    description: String,
    timeline: Vec<TimelineEntry>,
}

pub fn build(content_root: &Path, out_root: &Path) -> std::io::Result<String> {
    let bundle = render(content_root);
    let page = finalize(bundle);
    let out_dir = out_root.join("about");
    fs::create_dir_all(&out_dir)?;
    let content_dir = content_root.join("about");
    copy_assets(&content_dir, &out_dir)?;
    let out = out_dir.join("index.html");
    fs::write(&out, &page)?;
    Ok(out.display().to_string())
}

fn copy_assets(src: &Path, dst: &Path) -> io::Result<()> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        if entry.file_name() == "index.md" {
            continue;
        }
        if entry.file_type()?.is_file() {
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn render(content_root: &Path) -> Bundle {
    let index_md = content_root.join("about").join("index.md");
    let raw = fs::read_to_string(&index_md).unwrap_or_default();
    let (fm_yaml, body_md) = split_frontmatter(&raw);
    let fm: AboutFrontMatter = serde_yaml::from_str(fm_yaml)
        .unwrap_or_else(|e| panic!("invalid frontmatter in content/about/index.md: {e}"));

    let body_html = render_markdown(body_md.trim());
    let body_bundle = Bundle { html: body_html, css: String::new(), js: String::new() };
    let has_timeline = if fm.timeline.is_empty() { "" } else { "yes" };

    let entries: Vec<AboutEntry> = fm
        .timeline
        .iter()
        .map(|e| AboutEntry {
            date_display: format_date(&e.date),
            title: e.title.clone(),
            description: e.description.clone(),
        })
        .collect();

    let title = if fm.title.is_empty() { TITLE } else { &fm.title };
    let description = if fm.description.is_empty() { DESCRIPTION } else { &fm.description };

    let page = about(body_bundle, has_timeline, &entries);
    let canonical = format!("{SITE_URL}/about/");
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

fn format_date(raw: &str) -> String {
    let date = raw.get(..10).unwrap_or(raw);
    let mut parts = date.split('-');
    let (Some(y), m, d) = (parts.next(), parts.next(), parts.next()) else {
        return raw.to_string();
    };
    let month = match m {
        Some("01") => "January",
        Some("02") => "February",
        Some("03") => "March",
        Some("04") => "April",
        Some("05") => "May",
        Some("06") => "June",
        Some("07") => "July",
        Some("08") => "August",
        Some("09") => "September",
        Some("10") => "October",
        Some("11") => "November",
        Some("12") => "December",
        _ => return raw.to_string(),
    };
    match d {
        Some(day) => {
            let d = day.trim_start_matches('0');
            format!("{month} {d}, {y}")
        }
        None => format!("{month} {y}"),
    }
}

fn split_frontmatter(raw: &str) -> (&str, &str) {
    let rest = raw
        .strip_prefix("---\n")
        .or_else(|| raw.strip_prefix("---\r\n"))
        .unwrap_or_else(|| panic!("missing frontmatter start `---` in content/about/index.md"));
    let end = rest
        .find("\n---")
        .unwrap_or_else(|| panic!("missing frontmatter end `---` in content/about/index.md"));
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

    use super::{about, AboutEntry};

    #[preview("About")]
    fn default() -> Bundle {
        let body = Bundle {
            html: "<h1 class=\"about-title\">About</h1><p>A place that celebrates independent software and the people behind it.</p>".into(),
            css: String::new(),
            js: String::new(),
        };
        let entries = vec![
            AboutEntry {
                date_display: "January 2025".into(),
                title: "First spark".into(),
                description: "The idea took shape.".into(),
            },
            AboutEntry {
                date_display: "June 2025".into(),
                title: "Launch".into(),
                description: "Great Little Software goes live.".into(),
            },
        ];
        about(body, "yes", &entries)
    }
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

    #[test]
    fn parses_timeline_entries() {
        let y = "timeline:\n  - date: \"2025-01\"\n    title: Spark\n    description: Idea\n";
        let fm: AboutFrontMatter = serde_yaml::from_str(y).unwrap();
        assert_eq!(fm.timeline.len(), 1);
        assert_eq!(fm.timeline[0].date, "2025-01");
        assert_eq!(fm.timeline[0].title, "Spark");
        assert_eq!(fm.timeline[0].description, "Idea");
    }

    #[test]
    fn empty_frontmatter_defaults() {
        let y = "";
        let fm: AboutFrontMatter = serde_yaml::from_str(y).unwrap();
        assert!(fm.timeline.is_empty());
        assert_eq!(fm.title, "");
    }

    #[test]
    fn formats_full_date() {
        assert_eq!(format_date("2025-01-15"), "January 15, 2025");
    }

    #[test]
    fn formats_year_month() {
        assert_eq!(format_date("2025-01"), "January 2025");
    }
}