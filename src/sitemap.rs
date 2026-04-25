use std::fs;
use std::io;
use std::path::Path;

use crate::blog::{collect_posts, Post};
use crate::theme::SITE_URL;

pub fn build(content_root: &Path, out_root: &Path, include_drafts: bool) -> io::Result<String> {
    let posts = collect_posts(content_root, include_drafts)?;
    let xml = render(&posts);
    let out = out_root.join("sitemap.xml");
    fs::write(&out, xml)?;
    Ok(out.display().to_string())
}

fn render(posts: &[Post]) -> String {
    let blog_lastmod = posts
        .iter()
        .map(|p| lastmod_date(p.lastmod()))
        .max()
        .unwrap_or_default();

    let mut out = String::new();
    out.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    out.push_str("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");
    push_url(&mut out, &format!("{SITE_URL}/"), &blog_lastmod);
    push_url(&mut out, &format!("{SITE_URL}/blog/"), &blog_lastmod);
    for post in posts {
        push_url(&mut out, &post.canonical(), &lastmod_date(post.lastmod()));
    }
    out.push_str("</urlset>\n");
    out
}

fn push_url(out: &mut String, loc: &str, lastmod: &str) {
    out.push_str("  <url>\n");
    out.push_str(&format!("    <loc>{loc}</loc>\n"));
    if !lastmod.is_empty() {
        out.push_str(&format!("    <lastmod>{lastmod}</lastmod>\n"));
    }
    out.push_str("  </url>\n");
}

fn lastmod_date(raw: &str) -> String {
    raw.get(..10).unwrap_or(raw).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lastmod_trims_to_date() {
        assert_eq!(lastmod_date("2026-04-19T16:15:24Z"), "2026-04-19");
        assert_eq!(lastmod_date(""), "");
    }

    #[test]
    fn render_includes_home_blog_and_posts() {
        let xml = render(&[]);
        assert!(xml.contains("<loc>https://greatlittle.software/</loc>"));
        assert!(xml.contains("<loc>https://greatlittle.software/blog/</loc>"));
        assert!(xml.starts_with("<?xml"));
    }
}
