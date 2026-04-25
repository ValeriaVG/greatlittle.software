#[cfg(feature = "preview")]
mod previews {
    use macros::preview;

    use crate::html::Bundle;

    use super::super::latest;
    use crate::blog::{card, card_css, card_js};

    #[preview("Home/Latest")]
    fn default() -> Bundle {
        let mut html = String::new();
        html.push_str(&card(
            "example-post",
            "",
            "https://picsum.photos/seed/greatlittle/800/450",
            "Abstract placeholder cover",
            "2026-04-19",
            "April 19, 2026",
            "A Great Little Example Post",
            "A short description that gives a taste of what the article is about.",
        ));
        html.push_str(&card(
            "another",
            "",
            "https://picsum.photos/seed/another/800/450",
            "Abstract placeholder cover",
            "2026-03-02",
            "March 2, 2026",
            "Notes on Building Small",
            "Why constraints make better software, and the people who lean into them.",
        ));
        html.push_str(&card(
            "third",
            "",
            "",
            "",
            "2026-02-10",
            "February 10, 2026",
            "Field Report",
            "Short observations from the indie software world.",
        ));
        let cards = Bundle { html, css: card_css(), js: card_js() };
        latest(cards)
    }
}
