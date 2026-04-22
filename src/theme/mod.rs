use macros::html_template;

use crate::html::{template, Bundle};

html_template!(layout_template, "src/theme/layout");

pub const ROOT_CSS_PATH: &str = "src/theme/root.css";

pub fn root_css() -> String {
    std::fs::read_to_string(ROOT_CSS_PATH).unwrap_or_default()
}

pub fn layout(title: &str, description: &str, children: Bundle) -> Bundle {
    let mut out = layout_template(title, description, children);
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
