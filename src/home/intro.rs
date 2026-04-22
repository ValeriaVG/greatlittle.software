use macros::preview;

use crate::html::Bundle;

#[preview("Intro")]
fn basic() -> Bundle {
    Bundle {
        html: super::intro("Great Little Software."),
        css: super::intro_css(),
        js: super::intro_js(),
    }
}
