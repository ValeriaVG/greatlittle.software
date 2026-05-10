use std::fs;
use std::io;
use std::path::Path;

use crate::blog::{collect_posts, Post};
use crate::theme::SITE_URL;

pub fn build(content_root: &Path, out_root: &Path, include_drafts: bool) -> io::Result<Vec<String>> {
    let mut written = Vec::new();

    let home = home_md(content_root, include_drafts);
    let out = out_root.join("index.md");
    fs::write(&out, &home)?;
    written.push(out.display().to_string());

    let about_md = about_md(content_root);
    let out_dir = out_root.join("about");
    fs::create_dir_all(&out_dir)?;
    let out = out_dir.join("index.md");
    fs::write(&out, &about_md)?;
    written.push(out.display().to_string());

    let privacy_md = privacy_md(content_root);
    let out_dir = out_root.join("privacy");
    fs::create_dir_all(&out_dir)?;
    let out = out_dir.join("index.md");
    fs::write(&out, &privacy_md)?;
    written.push(out.display().to_string());

    let posts = collect_posts(content_root, include_drafts)?;
    if !posts.is_empty() || content_root.join("blog").exists() {
        let blog_index_md = blog_index_md(&posts);
        let out_dir = out_root.join("blog");
        fs::create_dir_all(&out_dir)?;
        let out = out_dir.join("index.md");
        fs::write(&out, &blog_index_md)?;
        written.push(out.display().to_string());

        for post in &posts {
            let post_md = post_md(post);
            let out_dir = out_root.join("blog").join(&post.slug);
            fs::create_dir_all(&out_dir)?;
            let out = out_dir.join("index.md");
            fs::write(&out, &post_md)?;
            written.push(out.display().to_string());
        }
    }

    Ok(written)
}

fn home_md(content_root: &Path, include_drafts: bool) -> String {
    let index_md = content_root.join("index.md");
    let raw = fs::read_to_string(&index_md).unwrap_or_default();
    let (fm_yaml, body_md) = split_frontmatter(&raw);

    let mut frontmatter = String::new();
    frontmatter.push_str("---\n");
    frontmatter.push_str("title: Great Little Software\n");
    frontmatter.push_str("description: Collection of indie apps, ideas, tooling and stories about larger-than-life people building tiny awesome products.\n");
    frontmatter.push_str(&format!("canonical: {SITE_URL}/\n"));
    frontmatter.push_str("---\n\n");

    let posts = collect_posts(content_root, include_drafts).unwrap_or_default();
    let mut body = String::new();

    if let Some(featured) = posts.first() {
        body.push_str("## Featured masterpiece\n\n");
        body.push_str(&format!("### [{}]({})\n\n", featured.title(), featured.canonical()));
        body.push_str(&format!("{}\n\n", featured.description()));
        if featured.has_cover() {
            body.push_str(&format!("![{}]({})\n\n", featured.cover_alt(), featured.cover_url()));
        }
    }

    if posts.len() > 1 {
        let recent = &posts[1..4.min(posts.len())];
        body.push_str("## Recent stories\n\n");
        for post in recent {
            body.push_str(&format!("### [{}]({})\n\n", post.title(), post.canonical()));
            body.push_str(&format!("{}\n\n", post.description()));
        }
    }

    if !body_md.trim().is_empty() {
        body.push_str(body_md.trim());
        body.push('\n');
    }

    body.push_str(&push_str_faq(fm_yaml));

    frontmatter.push_str(&body);
    frontmatter
}

fn push_str_faq(fm_yaml: &str) -> String {
    let fm: MdFaqFrontMatter = match serde_yaml::from_str(fm_yaml) {
        Ok(f) => f,
        Err(_) => return String::new(),
    };
    if fm.faq.is_empty() {
        return String::new();
    }
    let mut out = String::new();
    let title = if fm.faq_title.is_empty() {
        "Frequently Asked Questions"
    } else {
        &fm.faq_title
    };
    out.push_str(&format!("## {title}\n\n"));
    for item in &fm.faq {
        out.push_str(&format!("### {}\n\n{}\n\n", item.q, item.a));
    }
    out
}

fn about_md(content_root: &Path) -> String {
    let index_md = content_root.join("about").join("index.md");
    let raw = fs::read_to_string(&index_md).unwrap_or_default();
    let (fm_yaml, body_md) = split_frontmatter(&raw);
    let fm: MdAboutFrontMatter = serde_yaml::from_str(fm_yaml)
        .unwrap_or_else(|e| panic!("invalid frontmatter in content/about/index.md: {e}"));

    let title = if fm.title.is_empty() { "About" } else { &fm.title };
    let description = if fm.description.is_empty() {
        "The story behind Great Little Software."
    } else {
        &fm.description
    };

    let mut frontmatter = String::new();
    frontmatter.push_str("---\n");
    frontmatter.push_str(&format!("title: {}\n", yaml_escape(title)));
    frontmatter.push_str(&format!("description: {}\n", yaml_escape(description)));
    frontmatter.push_str(&format!("canonical: {SITE_URL}/about/\n"));
    frontmatter.push_str("---\n\n");

    let mut body = String::new();
    if !fm.timeline.is_empty() {
        body.push_str("## Timeline\n\n");
        for entry in &fm.timeline {
            body.push_str(&format!(
                "- **{}** ({}): {}\n",
                entry.title, entry.date, entry.description
            ));
        }
        body.push('\n');
    }
    body.push_str(body_md.trim());
    frontmatter.push_str(&body);
    frontmatter.push('\n');
    frontmatter
}

fn privacy_md(content_root: &Path) -> String {
    let index_md = content_root.join("privacy").join("index.md");
    let raw = fs::read_to_string(&index_md).unwrap_or_default();
    let (fm_yaml, body_md) = split_frontmatter(&raw);
    let fm: MdPrivacyFrontMatter = serde_yaml::from_str(fm_yaml)
        .unwrap_or_else(|e| panic!("invalid frontmatter in content/privacy/index.md: {e}"));

    let title = if fm.title.is_empty() { "Privacy Policy" } else { &fm.title };
    let description = if fm.description.is_empty() {
        "Privacy policy for Great Little Software."
    } else {
        &fm.description
    };

    let mut frontmatter = String::new();
    frontmatter.push_str("---\n");
    frontmatter.push_str(&format!("title: {}\n", yaml_escape(title)));
    frontmatter.push_str(&format!("description: {}\n", yaml_escape(description)));
    frontmatter.push_str(&format!("canonical: {SITE_URL}/privacy/\n"));
    frontmatter.push_str("---\n\n");
    frontmatter.push_str(body_md.trim());
    frontmatter.push('\n');
    frontmatter
}

fn blog_index_md(posts: &[Post]) -> String {
    let mut frontmatter = String::new();
    frontmatter.push_str("---\n");
    frontmatter.push_str("title: Blog | Great Little Software\n");
    frontmatter.push_str("description: Stories, notes and field reports about indie software.\n");
    frontmatter.push_str(&format!("canonical: {SITE_URL}/blog/\n"));
    frontmatter.push_str("---\n\n");

    let mut body = String::new();
    for post in posts {
        body.push_str(&format!("## [{}]({})\n\n", post.title(), post.canonical()));
        body.push_str(&format!("{}\n\n", post.description()));
        if post.has_cover() {
            body.push_str(&format!("![{}]({})\n\n", post.cover_alt(), post.cover_url()));
        }
    }
    frontmatter.push_str(&body);
    frontmatter
}

fn post_md(post: &Post) -> String {
    let mut frontmatter = String::new();
    frontmatter.push_str("---\n");
    frontmatter.push_str(&format!("title: {}\n", yaml_escape(post.title())));
    frontmatter.push_str(&format!("description: {}\n", yaml_escape(post.description())));
    frontmatter.push_str(&format!("canonical: {}\n", post.canonical()));
    if post.has_cover() {
        frontmatter.push_str(&format!("image: {}\n", post.cover_url()));
        frontmatter.push_str(&format!("image_alt: {}\n", yaml_escape(post.cover_alt())));
    }
    frontmatter.push_str(&format!("date: {}\n", post.created_at()));
    if !post.updated_at().is_empty() {
        frontmatter.push_str(&format!("updated: {}\n", post.updated_at()));
    }
    if !post.author_name().is_empty() {
        frontmatter.push_str(&format!("author: {}\n", yaml_escape(post.author_name())));
    }
    frontmatter.push_str("---\n\n");

    let md_path = post.source_dir().join("index.md");
    let raw = fs::read_to_string(&md_path).unwrap_or_default();
    let (_, body_md) = split_frontmatter(&raw);
    frontmatter.push_str(body_md.trim());
    frontmatter.push('\n');
    frontmatter
}

fn split_frontmatter(raw: &str) -> (&str, &str) {
    let rest = match raw.strip_prefix("---\n").or_else(|| raw.strip_prefix("---\r\n")) {
        Some(r) => r,
        None => return ("", raw),
    };
    let end = match rest.find("\n---") {
        Some(i) => i,
        None => return ("", raw),
    };
    let fm = &rest[..end];
    let after = &rest[end + 4..];
    let body = after
        .strip_prefix('\n')
        .or_else(|| after.strip_prefix("\r\n"))
        .unwrap_or(after);
    (fm, body)
}

fn yaml_escape(s: &str) -> String {
    if s.contains(':') || s.contains('#') || s.contains('"') || s.contains('\'') || s.contains('\n') {
        format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
    } else {
        s.to_string()
    }
}

use serde::Deserialize;

#[derive(Default, Deserialize)]
struct MdFaqFrontMatter {
    #[serde(default)]
    faq_title: String,
    #[serde(default)]
    faq: Vec<MdFaqEntry>,
}

#[derive(Default, Deserialize)]
struct MdFaqEntry {
    q: String,
    a: String,
}

#[derive(Default, Deserialize)]
struct MdAboutFrontMatter {
    #[serde(default)]
    title: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    timeline: Vec<MdTimelineEntry>,
}

#[derive(Default, Deserialize)]
struct MdTimelineEntry {
    date: String,
    title: String,
    #[serde(default)]
    description: String,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct MdPrivacyFrontMatter {
    title: String,
    description: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_frontmatter_works() {
        let raw = "---\ntitle: Hi\n---\nBody\n";
        let (fm, body) = split_frontmatter(raw);
        assert_eq!(fm, "title: Hi");
        assert_eq!(body, "Body\n");
    }

    #[test]
    fn split_frontmatter_no_frontmatter() {
        let raw = "Just body text\n";
        let (fm, body) = split_frontmatter(raw);
        assert_eq!(fm, "");
        assert_eq!(body, "Just body text\n");
    }

    #[test]
    fn yaml_escape_plain() {
        assert_eq!(yaml_escape("hello"), "hello");
    }

    #[test]
    fn yaml_escape_with_colon() {
        assert_eq!(yaml_escape("hello: world"), "\"hello: world\"");
    }

    #[test]
    fn yaml_escape_with_quotes() {
        assert_eq!(yaml_escape("say \"hi\""), "\"say \\\"hi\\\"\"");
    }
}