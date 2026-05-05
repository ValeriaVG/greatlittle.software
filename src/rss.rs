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
    let last_build = now_rfc822();

    let mut out = String::new();
    out.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    out.push_str("<rss version=\"2.0\" xmlns:atom=\"http://www.w3.org/2005/Atom\">\n");
    out.push_str("  <channel>\n");
    out.push_str(&format!("    <title>{}</title>\n", escape_html(FEED_TITLE)));
    out.push_str(&format!("    <description>{}</description>\n", escape_html(FEED_DESCRIPTION)));
    out.push_str(&format!("    <link>{SITE_URL}/blog/</link>\n"));
    out.push_str(&format!("    <lastBuildDate>{last_build}</lastBuildDate>\n"));
    out.push_str("    <atom:link href=\"https://greatlittle.software/rss.xml\" rel=\"self\" type=\"application/rss+xml\"/>\n");
    out.push_str(&format!("    <image>\n      <url>{SITE_URL}/assets/icon.png</url>\n      <title>{}</title>\n      <link>{SITE_URL}/blog/</link>\n    </image>\n", escape_html(FEED_TITLE)));
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
    let time = raw
        .get(11..19)
        .map(|t| format!("{t} +0000"))
        .unwrap_or_else(|| "00:00:00 +0000".into());
    format!("{d} {month} {y} {time}")
}

fn now_rfc822() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let mut remaining = secs as i64;
    let mut year: u32 = 1970;
    loop {
        let yd: i64 = if is_leap(year) { 366 } else { 365 };
        if remaining < yd * 86_400 {
            break;
        }
        remaining -= yd * 86_400;
        year += 1;
    }
    let is_ly = is_leap(year);
    let days_in_month: [i64; 12] = if is_ly {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    let mut month_idx: usize = 0;
    for (i, &dim) in days_in_month.iter().enumerate() {
        if remaining < dim * 86_400 {
            month_idx = i;
            break;
        }
        remaining -= dim * 86_400;
    }
    let day = remaining / 86_400 + 1;
    remaining %= 86_400;
    let hour = remaining / 3600;
    remaining %= 3600;
    let minute = remaining / 60;
    let second = remaining % 60;
    let month_name = match month_idx {
        0 => "Jan",
        1 => "Feb",
        2 => "Mar",
        3 => "Apr",
        4 => "May",
        5 => "Jun",
        6 => "Jul",
        7 => "Aug",
        8 => "Sep",
        9 => "Oct",
        10 => "Nov",
        11 => "Dec",
        _ => "Jan",
    };
    format!("{day:02} {month_name} {year} {hour:02}:{minute:02}:{second:02} +0000")
}

fn is_leap(y: u32) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rss_date_format() {
        assert_eq!(rss_date("2026-04-19T16:15:24Z"), "19 Apr 2026 16:15:24 +0000");
        assert_eq!(rss_date("2026-04-19"), "19 Apr 2026 00:00:00 +0000");
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