use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use macros::html_template;
use pulldown_cmark::{html as cmark_html, Options, Parser};

use crate::home::layout;
use crate::html::{escape_html, finalize, json_escape, Fragment};

const SITE_URL: &str = "https://greatlittle.software";
const SITE_NAME: &str = "Great Little Software";

html_template!(article, "src/blog/article");
html_template!(index, "src/blog/index");
html_template!(card, "src/blog/card");

pub struct Post {
    pub slug: String,
    pub dir: PathBuf,
    pub title: String,
    pub title_raw: String,
    pub description: String,
    pub description_raw: String,
    pub keywords: Vec<String>,
    pub is_draft: bool,
    pub created_at: String,
    pub updated_at: String,
    pub author: String,
    pub date_display: String,
    pub cover_src: String,
    pub cover_src_raw: String,
    pub cover_alt: String,
    pub body_html: String,
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
        let post = parse_post(&slug, dir, &raw);
        if post.is_draft && !include_drafts {
            continue;
        }
        posts.push(post);
    }
    posts.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    let mut written = Vec::new();
    for post in &posts {
        let post_out = blog_out.join(&post.slug);
        fs::create_dir_all(&post_out)?;
        copy_assets(&post.dir, &post_out)?;

        let canonical = format!("{SITE_URL}/blog/{}/", post.slug);
        let image = if post.cover_src_raw.is_empty() {
            String::new()
        } else {
            format!("{SITE_URL}/blog/{}/{}", post.slug, post.cover_src_raw)
        };
        let author = if post.author.is_empty() { SITE_NAME.to_string() } else { post.author.clone() };
        let updated = if post.updated_at.is_empty() { &post.created_at } else { &post.updated_at };
        let keywords = post.keywords.join(", ");

        let ld_title = json_escape(&post.title_raw);
        let ld_description = json_escape(&post.description_raw);
        let ld_image = json_escape(&image);
        let ld_published = json_escape(&post.created_at);
        let ld_modified = json_escape(updated);
        let ld_url = json_escape(&canonical);
        let ld_keywords = json_escape(&keywords);
        let ld_author = json_escape(&author);

        let body = Fragment {
            html: post.body_html.clone(),
            css: String::new(),
            js: String::new(),
        };
        let draft_marker = if post.is_draft { "draft" } else { "" };
        let art = article(
            &ld_title,
            &ld_description,
            &ld_image,
            &ld_published,
            &ld_modified,
            &ld_url,
            &ld_keywords,
            &ld_author,
            draft_marker,
            &post.created_at,
            &post.date_display,
            &post.title,
            &post.description,
            &post.cover_src,
            &post.cover_alt,
            body,
        );
        let page = layout(&post.title, &post.description, art);
        let html = finalize(page);
        let out = post_out.join("index.html");
        fs::write(&out, html)?;
        written.push(out.display().to_string());
    }

    let mut cards_html = String::new();
    let mut cards_css = String::new();
    for post in &posts {
        let draft_marker = if post.is_draft { "draft" } else { "" };
        let c = card(
            &post.slug,
            draft_marker,
            &post.cover_src,
            &post.cover_alt,
            &post.created_at,
            &post.date_display,
            &post.title,
            &post.description,
        );
        cards_html.push_str(&c.html);
        if !c.css.is_empty() && cards_css.is_empty() {
            cards_css = c.css;
        }
    }
    let cards = Fragment {
        html: cards_html,
        css: cards_css,
        js: String::new(),
    };
    const BLOG_TITLE: &str = "Blog";
    const BLOG_DESCRIPTION: &str = "Stories, notes and field reports about indie software.";
    let idx = index(BLOG_TITLE, BLOG_DESCRIPTION, cards);
    let page_title = format!("{BLOG_TITLE} | {SITE_NAME}");
    let page = layout(&page_title, BLOG_DESCRIPTION, idx);
    let html = finalize(page);
    let out = blog_out.join("index.html");
    fs::write(&out, html)?;
    written.push(out.display().to_string());

    Ok(written)
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

fn parse_post(slug: &str, dir: PathBuf, raw: &str) -> Post {
    let (fm, body_md) = split_frontmatter(raw);
    let fm = parse_frontmatter(fm);
    let body_html = render_markdown(body_md);
    let date_display = format_iso_date(&fm.created_at);
    Post {
        slug: slug.to_string(),
        dir,
        title: escape_html(&fm.title),
        title_raw: fm.title.clone(),
        description: escape_html(&fm.description),
        description_raw: fm.description.clone(),
        keywords: fm.keywords,
        is_draft: fm.is_draft,
        date_display: escape_html(&date_display),
        created_at: escape_html(&fm.created_at),
        updated_at: escape_html(&fm.updated_at),
        author: fm.author.clone(),
        cover_src: escape_html(&fm.cover_src),
        cover_src_raw: fm.cover_src.clone(),
        cover_alt: escape_html(&fm.cover_alt),
        body_html,
    }
}

fn split_frontmatter(raw: &str) -> (&str, &str) {
    let rest = raw
        .strip_prefix("---\n")
        .or_else(|| raw.strip_prefix("---\r\n"))
        .expect("missing frontmatter start `---`");
    let end = rest
        .find("\n---")
        .expect("missing frontmatter end `---`");
    let fm = &rest[..end];
    let after = &rest[end + 4..];
    let body = after
        .strip_prefix('\n')
        .or_else(|| after.strip_prefix("\r\n"))
        .unwrap_or(after);
    (fm, body)
}

#[derive(Default)]
struct FrontMatter {
    title: String,
    description: String,
    keywords: Vec<String>,
    is_draft: bool,
    created_at: String,
    updated_at: String,
    author: String,
    cover_src: String,
    cover_alt: String,
}

fn parse_frontmatter(yaml: &str) -> FrontMatter {
    let mut fm = FrontMatter::default();
    let mut group: Option<String> = None;
    for line in yaml.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let indented = line.starts_with(' ') || line.starts_with('\t');
        if indented {
            if let Some(g) = &group {
                if let Some((k, v)) = split_kv(line.trim_start()) {
                    match (g.as_str(), k.as_str()) {
                        ("cover", "src") => fm.cover_src = unquote(&v),
                        ("cover", "alt") => fm.cover_alt = unquote(&v),
                        _ => {}
                    }
                }
            }
            continue;
        }
        group = None;
        let Some((k, v)) = split_kv(line) else { continue };
        let v_trim = v.trim();
        match k.as_str() {
            "title" => fm.title = unquote(v_trim),
            "description" => fm.description = unquote(v_trim),
            "is_draft" => fm.is_draft = v_trim == "true",
            "created_at" => fm.created_at = unquote(v_trim),
            "updated_at" => fm.updated_at = unquote(v_trim),
            "author" => fm.author = unquote(v_trim),
            "keywords" => fm.keywords = parse_array(v_trim),
            "cover" => {
                if v_trim.is_empty() {
                    group = Some("cover".to_string());
                }
            }
            _ => {}
        }
    }
    fm
}

fn split_kv(line: &str) -> Option<(String, String)> {
    let colon = line.find(':')?;
    let k = line[..colon].trim().to_string();
    let v = line[colon + 1..].trim().to_string();
    Some((k, v))
}

fn unquote(s: &str) -> String {
    let s = s.trim();
    if s.len() >= 2 {
        let bytes = s.as_bytes();
        if (bytes[0] == b'"' && bytes[s.len() - 1] == b'"')
            || (bytes[0] == b'\'' && bytes[s.len() - 1] == b'\'')
        {
            return s[1..s.len() - 1].to_string();
        }
    }
    s.to_string()
}

fn parse_array(s: &str) -> Vec<String> {
    let s = s.trim();
    let inner = s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).unwrap_or(s);
    inner
        .split(',')
        .map(|item| unquote(item.trim()))
        .filter(|item| !item.is_empty())
        .collect()
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
        let fm = parse_frontmatter(y);
        assert_eq!(fm.title, "T");
        assert_eq!(fm.cover_src, "a.png");
        assert_eq!(fm.cover_alt, "alt text");
    }

    #[test]
    fn formats_iso_date() {
        assert_eq!(format_iso_date("2026-04-19T16:15:24Z"), "April 19, 2026");
    }

    #[test]
    fn parses_keyword_array() {
        assert_eq!(parse_array("[a, b, \"c d\"]"), vec!["a", "b", "c d"]);
    }
}
