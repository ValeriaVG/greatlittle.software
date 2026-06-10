use std::fs;
use std::io;
use std::path::Path;

use macros::html_template;
use serde::Deserialize;

use crate::html::{Bundle, finalize, template};
use crate::markdown::{render_markdown, split_frontmatter};
use crate::theme::{SITE_URL, layout};

html_template!(privacy, "src/privacy");

const TITLE: &str = "Privacy Policy";
const DESCRIPTION: &str = "Privacy policy for Great Little Software.";

#[derive(Default, Deserialize)]
#[serde(default)]
struct PrivacyFrontMatter {
    title: String,
    description: String,
}

pub fn build(content_root: &Path, out_root: &Path) -> std::io::Result<String> {
    let bundle = render(content_root).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let page = finalize(bundle);
    let out_dir = out_root.join("privacy");
    fs::create_dir_all(&out_dir)?;
    let out = out_dir.join("index.html");
    fs::write(&out, &page)?;
    Ok(out.display().to_string())
}

fn render(content_root: &Path) -> Result<Bundle, String> {
    let index_md = content_root.join("privacy").join("index.md");
    let raw = fs::read_to_string(&index_md).unwrap_or_default();
    let (fm_yaml, body_md) =
        split_frontmatter(&raw).ok_or("missing frontmatter in content/privacy/index.md")?;
    let _fm: PrivacyFrontMatter = yaml_serde::from_str(fm_yaml)
        .map_err(|e| format!("invalid frontmatter in content/privacy/index.md: {e}"))?;

    let body_html = render_markdown(body_md.trim());
    let body_bundle = Bundle {
        html: body_html,
        css: String::new(),
        js: String::new(),
    };

    let title = if _fm.title.is_empty() {
        TITLE
    } else {
        &_fm.title
    };
    let description = if _fm.description.is_empty() {
        DESCRIPTION
    } else {
        &_fm.description
    };

    let page = privacy(body_bundle);
    let canonical = format!("{SITE_URL}/privacy/");
    Ok(layout(title, description, &canonical, page))
}
