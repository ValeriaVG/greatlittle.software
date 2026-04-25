use macros::html_template;

use crate::html::{template, Bundle};

mod header;

html_template!(layout_template, "src/theme/layout");
html_template!(header, "src/theme/header");

pub const SITE_URL: &str = "https://greatlittle.software";
pub const ROOT_CSS_PATH: &str = "src/theme/root.css";

pub fn root_css() -> String {
    std::fs::read_to_string(ROOT_CSS_PATH).unwrap_or_default()
}

fn current_year() -> u32 {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let mut days = (secs / 86_400) as i64;
    let mut year: u32 = 1970;
    loop {
        let yd: i64 = if is_leap(year) { 366 } else { 365 };
        if days < yd {
            return year;
        }
        days -= yd;
        year += 1;
    }
}

fn is_leap(y: u32) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}

pub fn layout(title: &str, description: &str, canonical: &str, children: Bundle) -> Bundle {
    layout_with(title, description, canonical, false, children)
}

pub fn home_layout(title: &str, description: &str, canonical: &str, children: Bundle) -> Bundle {
    layout_with(title, description, canonical, true, children)
}

fn layout_with(title: &str, description: &str, canonical: &str, is_home: bool, children: Bundle) -> Bundle {
    let year = current_year().to_string();
    let (is_home_v, is_inner_v) = if is_home { ("1", "") } else { ("", "1") };
    let header_bundle = Bundle {
        html: header(is_home_v, is_inner_v),
        css: header_css(),
        js: header_js(),
    };
    let mut out = layout_template(title, description, canonical, header_bundle, children, &year);
    let root = root_css();
    if !root.is_empty() {
        if out.css.is_empty() {
            out.css = root;
        } else {
            let mut combined = String::with_capacity(root.len() + 1 + out.css.len());
            combined.push_str(&root);
            combined.push('\n');
            combined.push_str(&out.css);
            out.css = combined;
        }
    }
    out
}
