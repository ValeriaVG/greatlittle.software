use std::fs;
use std::io;
use std::path::Path;

use crate::blog::{collect_posts, Post};
use crate::theme::SITE_URL;
use crate::html::escape_html;

const FEED_TITLE: &str = "Great Little Software Blog";
const FEED_DESCRIPTION: &str = "Stories, notes and field reports about indie software.";

pub fn build(content_root: &Path, out_root: &Path, include_drafts: bool) -> io::Result<String> {
    let posts = collect_posts(content_root, include_drafts)?;
    let xml = render(&posts);
    let out = out_root.join("rss.xml");
    fs::write(&out, xml)?;
    Ok(out.display().to_string())
}

fn render(posts: &[Post]) -> String {
    let last_build = posts
        .iter()
        .map(|p| rss_date(p.lastmod()))
        .max()
        .unwrap_or_default();

    let mut out = String::new();
    out.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    out.push_str("<rss version=\"2.0\" xmlns:atom=\"http://www.w3.org/2005/Atom\">\n");
    out.push_str("  <channel>\n");
    out.push_str(&format!("    <title>{}</title>\n", escape_html(FEED_TITLE)));
    out.push_str(&format!("    <description>{}</description>\n", escape_html(FEED_DESCRIPTION)));
    out.push_str(&format!("    <link>{SITE_URL}/blog/</link>\n"));
    out.push_str(&format!("    <lastBuildDate>{last_build}</lastBuildDate>\n"));
    out.push_str("    <atom:link href=\"https://greatlittle.software/rss.xml\" rel=\"self\" type=\"application/rss+xml\"/>\n");
    for post in posts {
        push_item(&mut out, post);
    }
    out.push_str("  </channel>\n");
    out.push_str("</rss>\n");
    out
}

fn push_item(out: &mut String, post: &Post) {
    out.push_str("    <item>\n");
    out.push_str(&format!("      <title>{}</title>\n", escape_html(post.title())));
    out.push_str(&format!("      <link>{}</link>\n", post.canonical()));
    out.push_str(&format!("      <guid>{}</guid>\n", post.canonical()));
    out.push_str(&format!("      <pubDate>{}</pubDate>\n", rss_date(post.lastmod())));
    let content = item_content(post);
    out.push_str(&format!("      <description><![CDATA[{content}]]></description>\n"));
    out.push_str("    </item>\n");
}

fn item_content(post: &Post) -> String {
    let mut out = String::new();
    let cover = post.cover_url();
    if !cover.is_empty() {
        out.push_str(&format!(
            "<img src=\"{}\" alt=\"{}\"/>",
            cover,
            escape_html(post.cover_alt())
        ));
    }
    out.push_str(post.body_html());
    out
}

fn rss_date(raw: &str) -> String {
    let date = raw.get(..10).unwrap_or(raw);
    let mut parts = date.split('-');
    let (Some(y), Some(m), Some(d)) = (parts.next(), parts.next(), parts.next()) else {
        return raw.to_string();
    };
    let month = match m {
        "01" => "Jan",
        "02" => "Feb",
        "03" => "Mar",
        "04" => "Apr",
        "05" => "May",
        "06" => "Jun",
        "07" => "Jul",
        "08" => "Aug",
        "09" => "Sep",
        "10" => "Oct",
        "11" => "Nov",
        "12" => "Dec",
        _ => return raw.to_string(),
    };
    format!("{d} {month} {y} 00:00:00 +0000")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rss_date_format() {
        assert_eq!(rss_date("2026-04-19T16:15:24Z"), "19 Apr 2026 00:00:00 +0000");
        assert_eq!(rss_date(""), "");
    }

    #[test]
    fn render_produces_valid_rss() {
        let xml = render(&[]);
        assert!(xml.contains("<rss version=\"2.0\""));
        assert!(xml.contains("<channel>"));
        assert!(xml.contains(&format!("<link>{SITE_URL}/blog/</link>")));
        assert!(xml.contains("atom:link"));
    }
}