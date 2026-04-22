use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use macros::html_template;
use pulldown_cmark::{html as cmark_html, Options, Parser};
use serde::Deserialize;

use crate::home::layout;
use crate::html::{finalize, template, Fragment};

const SITE_URL: &str = "https://greatlittle.software";
const SITE_NAME: &str = "Great Little Software";
const BLOG_TITLE: &str = "Blog";
const BLOG_DESCRIPTION: &str = "Stories, notes and field reports about indie software.";

// The `article` macro also picks up `article.ld.json` and emits it as JSON-LD
// before the article body. Keep the two in sync when adding fields.
html_template!(article, "src/blog/article");
html_template!(breadcrumbs, "src/blog/breadcrumbs");
html_template!(index, "src/blog/index");
html_template!(card, "src/blog/card");

#[derive(Default, Deserialize)]
#[serde(default)]
struct Cover {
    src: String,
    alt: String,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct FrontMatter {
    title: String,
    description: String,
    keywords: Vec<String>,
    is_draft: bool,
    created_at: String,
    updated_at: String,
    author: String,
    cover: Cover,
}

pub struct Post {
    slug: String,
    dir: PathBuf,
    fm: FrontMatter,
    date_display: String,
    body_html: String,
}

impl Post {
    fn canonical(&self) -> String {
        format!("{SITE_URL}/blog/{}/", self.slug)
    }
    fn cover_url(&self) -> String {
        if self.fm.cover.src.is_empty() {
            String::new()
        } else {
            format!("{SITE_URL}/blog/{}/{}", self.slug, self.fm.cover.src)
        }
    }
    fn author(&self) -> String {
        if self.fm.author.is_empty() { SITE_NAME.into() } else { self.fm.author.clone() }
    }
    fn updated(&self) -> &str {
        if self.fm.updated_at.is_empty() { &self.fm.created_at } else { &self.fm.updated_at }
    }
    fn draft_marker(&self) -> &'static str {
        if self.fm.is_draft { "draft" } else { "" }
    }
}

fn crumb(href: &str, label: impl Into<String>) -> BreadcrumbsItem {
    BreadcrumbsItem { href: href.into(), label: label.into(), current: String::new() }
}

fn current_crumb(label: impl Into<String>) -> BreadcrumbsItem {
    BreadcrumbsItem { href: String::new(), label: label.into(), current: "page".into() }
}

pub fn build(content_root: &Path, out_root: &Path, include_drafts: bool) -> io::Result<Vec<String>> {
    let blog_src = content_root.join("blog");
    let blog_out = out_root.join("blog");
    if !blog_src.exists() {
        return Ok(Vec::new());
    }
    fs::create_dir_all(&blog_out)?;

    let mut posts: Vec<Post> = Vec::new();
    for entry in fs::read_dir(&blog_src)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let slug = entry.file_name().to_string_lossy().into_owned();
        let dir = entry.path();
        let index_md = dir.join("index.md");
        if !index_md.exists() {
            continue;
        }
        let raw = fs::read_to_string(&index_md)?;
        let post = parse_post(slug, dir, &raw);
        if post.fm.is_draft && !include_drafts {
            continue;
        }
        posts.push(post);
    }
    posts.sort_by(|a, b| b.fm.created_at.cmp(&a.fm.created_at));

    let mut written = Vec::new();
    let mut cards_html = String::new();
    let mut cards_css = String::new();
    for post in &posts {
        let post_out = blog_out.join(&post.slug);
        fs::create_dir_all(&post_out)?;
        copy_assets(&post.dir, &post_out)?;

        let out = post_out.join("index.html");
        fs::write(&out, render_post_page(post))?;
        written.push(out.display().to_string());

        let c = card_for(post);
        if cards_css.is_empty() {
            cards_css = c.css;
        }
        cards_html.push_str(&c.html);
    }

    let cards = Fragment { html: cards_html, css: cards_css, js: String::new() };
    let crumbs = breadcrumbs(&[crumb("/", "Home"), current_crumb("Blog")]);
    let idx = index(crumbs, BLOG_TITLE, BLOG_DESCRIPTION, cards);
    let page_title = format!("{BLOG_TITLE} | {SITE_NAME}");
    let page = layout(&page_title, BLOG_DESCRIPTION, idx);
    let out = blog_out.join("index.html");
    fs::write(&out, finalize(page))?;
    written.push(out.display().to_string());

    Ok(written)
}

fn render_post_page(post: &Post) -> String {
    let article_data = ArticleArticle {
        title: post.fm.title.clone(),
        url: post.canonical(),
        description: post.fm.description.clone(),
        author: post.author(),
    };
    let cover_data = ArticleCover {
        src: post.fm.cover.src.clone(),
        alt: post.fm.cover.alt.clone(),
        url: post.cover_url(),
    };
    let body = Fragment {
        html: post.body_html.clone(),
        css: String::new(),
        js: String::new(),
    };
    let crumbs = breadcrumbs(&[
        crumb("/", "Home"),
        crumb("/blog/", "Blog"),
        current_crumb(post.fm.title.clone()),
    ]);
    let keywords = post.fm.keywords.join(", ");
    let art = article(
        crumbs,
        post.draft_marker(),
        &post.fm.created_at,
        &post.date_display,
        &article_data,
        &cover_data,
        body,
        post.updated(),
        &keywords,
    );
    finalize(layout(&post.fm.title, &post.fm.description, art))
}

fn card_for(post: &Post) -> Fragment {
    card(
        &post.slug,
        post.draft_marker(),
        &post.fm.cover.src,
        &post.fm.cover.alt,
        &post.fm.created_at,
        &post.date_display,
        &post.fm.title,
        &post.fm.description,
    )
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

fn parse_post(slug: String, dir: PathBuf, raw: &str) -> Post {
    let (fm_yaml, body_md) = split_frontmatter(raw);
    let fm: FrontMatter = serde_yaml::from_str(fm_yaml)
        .unwrap_or_else(|e| panic!("invalid frontmatter in {slug}: {e}"));
    let body_html = render_markdown(body_md);
    let date_display = format_iso_date(&fm.created_at);
    Post { slug, dir, fm, date_display, body_html }
}

fn split_frontmatter(raw: &str) -> (&str, &str) {
    let rest = raw
        .strip_prefix("---\n")
        .or_else(|| raw.strip_prefix("---\r\n"))
        .expect("missing frontmatter start `---`");
    let end = rest.find("\n---").expect("missing frontmatter end `---`");
    let fm = &rest[..end];
    let after = &rest[end + 4..];
    let body = after
        .strip_prefix('\n')
        .or_else(|| after.strip_prefix("\r\n"))
        .unwrap_or(after);
    (fm, body)
}

fn render_markdown(md: &str) -> String {
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

fn format_iso_date(iso: &str) -> String {
    let date = iso.get(..10).unwrap_or(iso);
    let mut parts = date.split('-');
    let (Some(y), Some(m), Some(d)) = (parts.next(), parts.next(), parts.next()) else {
        return iso.to_string();
    };
    let month = match m {
        "01" => "January",
        "02" => "February",
        "03" => "March",
        "04" => "April",
        "05" => "May",
        "06" => "June",
        "07" => "July",
        "08" => "August",
        "09" => "September",
        "10" => "October",
        "11" => "November",
        "12" => "December",
        _ => return iso.to_string(),
    };
    let day = d.trim_start_matches('0');
    format!("{month} {day}, {y}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_frontmatter() {
        let raw = "---\ntitle: Hi\n---\nBody here\n";
        let (fm, body) = split_frontmatter(raw);
        assert_eq!(fm, "title: Hi");
        assert_eq!(body, "Body here\n");
    }

    #[test]
    fn parses_nested_cover() {
        let y = "title: \"T\"\ncover:\n    src: a.png\n    alt: alt text\n";
        let fm: FrontMatter = serde_yaml::from_str(y).unwrap();
        assert_eq!(fm.title, "T");
        assert_eq!(fm.cover.src, "a.png");
        assert_eq!(fm.cover.alt, "alt text");
    }

    #[test]
    fn parses_keyword_array() {
        let y = "keywords: [a, b, \"c d\"]\n";
        let fm: FrontMatter = serde_yaml::from_str(y).unwrap();
        assert_eq!(fm.keywords, vec!["a", "b", "c d"]);
    }

    #[test]
    fn ignores_unknown_fields() {
        let y = "title: T\nlink: https://example.com/\n";
        let fm: FrontMatter = serde_yaml::from_str(y).unwrap();
        assert_eq!(fm.title, "T");
    }

    #[test]
    fn formats_iso_date() {
        assert_eq!(format_iso_date("2026-04-19T16:15:24Z"), "April 19, 2026");
    }
}
