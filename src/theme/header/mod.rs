#[cfg(feature = "preview")]
mod previews {
    use macros::preview;

    use crate::html::Bundle;

    #[preview("Header")]
    fn basic() -> Bundle {
        Bundle {
            html: super::super::header("", "1"),
            css: super::super::header_css(),
            js: super::super::header_js(),
        }
    }

    #[preview("Header (home)")]
    fn home() -> Bundle {
        Bundle {
            html: super::super::header("1", ""),
            css: super::super::header_css(),
            js: super::super::header_js(),
        }
    }
}
