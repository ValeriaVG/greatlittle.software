use macros::html_template;

use crate::html::{template, Bundle};
use crate::theme::layout;

mod intro;

html_template!(intro, "src/home/intro");

pub fn render() -> Bundle {
    const TITLE: &str = "Great Little Software";
    let body = Bundle {
        html: intro(&(TITLE.to_owned() + ".")),
        css: intro_css(),
        js: intro_js(),
    };
    layout(
        TITLE,
        "Collection of indie apps, ideas, tooling, stories and articles about larger-than-life people building tiny awesome products.",
        body,
    )
}
