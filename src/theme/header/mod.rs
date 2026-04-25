#[cfg(feature = "preview")]
mod previews {
    use macros::preview;

    use crate::html::Bundle;

    #[preview("Header")]
    fn basic() -> Bundle {
        Bundle {
            html: super::super::header(),
            css: super::super::header_css(),
            js: super::super::header_js(),
        }
    }
}
