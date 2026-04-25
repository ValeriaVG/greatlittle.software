use std::path::Path;

use macros::html_template;

use crate::blog;
use crate::html::{template, Bundle};
use crate::theme::{home_layout, SITE_URL};

mod latest;

html_template!(latest, "src/home/latest");

const TITLE: &str = "Great Little Software";
const DESCRIPTION: &str = "Collection of indie apps, ideas, tooling and stories about larger-than-life people building tiny awesome products.";
pub fn render(content_root: &Path, include_drafts: bool) -> Bundle {
    let posts = blog::collect_posts(content_root, include_drafts).unwrap_or_default();

    let mut body = Bundle { html: String::new(), css: String::new(), js: String::new() };

    if let Some(featured) = posts.first() {
        let cards = blog::cards_bundle(std::slice::from_ref(featured));
        body = merge(body, latest(cards));
    }

    body = merge(body, Bundle {
        html: blog::newsletter(),
        css: blog::newsletter_css(),
        js: blog::newsletter_js(),
    });

    home_layout(TITLE, DESCRIPTION, &format!("{SITE_URL}/"), body)
}

fn merge(a: Bundle, b: Bundle) -> Bundle {
    let html = a.html + &b.html;
    let css = join(a.css, b.css);
    let js = join(a.js, b.js);
    Bundle { html, css, js }
}

fn join(mut a: String, b: String) -> String {
    if b.is_empty() {
        return a;
    }
    if !a.is_empty() {
        a.push('\n');
    }
    a.push_str(&b);
    a
}
