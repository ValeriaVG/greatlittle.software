use macros::preview;

use crate::html::Bundle;

#[preview("Card/Default/Full")]
fn with_image() -> Bundle {
    Bundle {
        html: super::card(
            "example-post",
            "",
            "https://picsum.photos/seed/greatlittle/800/450",
            "Abstract placeholder cover",
            "2026-04-19",
            "April 19, 2026",
            "A Great Little Example Post",
            "A short description that gives a taste of what the article is about.",
        ),
        css: super::card_css(),
        js: super::card_js(),
    }
}

#[preview("Card/Default/Compact")]
fn without_image() -> Bundle {
    Bundle {
        html: super::card(
            "another-post",
            "",
            "",
            "",
            "2026-03-02",
            "March 2, 2026",
            "A Draft Without a Cover",
            "When there is no cover image the card collapses to just the text block.",
        ),
        css: super::card_css(),
        js: super::card_js(),
    }
}


#[preview("Card/Draft/Compact")]
fn draft_without_image() -> Bundle {
    Bundle {
        html: super::card(
            "another-post",
            "draft",
            "",
            "",
            "2026-03-02",
            "March 2, 2026",
            "A Draft Without a Cover",
            "When there is no cover image the card collapses to just the text block.",
        ),
        css: super::card_css(),
        js: super::card_js(),
    }
}


#[preview("Card/Draft/Full")]
fn draft_with_image() -> Bundle {
    Bundle {
        html: super::card(
            "example-post",
            "draft",
            "https://picsum.photos/seed/greatlittle/800/450",
            "Abstract placeholder cover",
            "2026-04-19",
            "April 19, 2026",
            "A Great Little Example Post",
            "A short description that gives a taste of what the article is about.",
        ),
        css: super::card_css(),
        js: super::card_js(),
    }
}
