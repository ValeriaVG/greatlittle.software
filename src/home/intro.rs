#[cfg(feature = "preview")]
mod previews {
    use macros::preview;

    use crate::html::Bundle;

    #[preview("Intro")]
    fn basic() -> Bundle {
        Bundle {
            html: super::super::intro("Great Little Software."),
            css: super::super::intro_css(),
            js: super::super::intro_js(),
        }
    }
}
