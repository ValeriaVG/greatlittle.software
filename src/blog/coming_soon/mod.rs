#[cfg(feature = "preview")]
mod previews {
    use macros::preview;

    use crate::html::Bundle;

    use super::super::{coming_soon, coming_soon_css, coming_soon_js};

    #[preview("Blog index/Coming soon card")]
    fn default() -> Bundle {
        Bundle {
            html: format!("<ul class=\"post-list\">{}</ul>", coming_soon()),
            css: coming_soon_css(),
            js: coming_soon_js(),
        }
    }
}
