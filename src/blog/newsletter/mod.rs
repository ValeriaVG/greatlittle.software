#[cfg(feature = "preview")]
mod previews {
    use macros::preview;

    use crate::html::Bundle;

    use super::super::{newsletter, newsletter_css, newsletter_js};

    #[preview("Newsletter/Default")]
    fn default() -> Bundle {
        Bundle { html: newsletter(), css: newsletter_css(), js: newsletter_js() }
    }
}
