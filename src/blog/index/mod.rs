#[cfg(feature = "preview")]
mod previews {
    use macros::preview;

    use crate::html::Bundle;

    use super::super::{
        BreadcrumbsItem, breadcrumbs, breadcrumbs_css, breadcrumbs_js, card, card_css, card_js,
        index, newsletter, newsletter_css, newsletter_js,
    };

    fn crumbs() -> Bundle {
        let items = [
            BreadcrumbsItem {
                href: "/".into(),
                label: "Home".into(),
                current: String::new(),
            },
            BreadcrumbsItem {
                href: String::new(),
                label: "Blog".into(),
                current: "page".into(),
            },
        ];
        Bundle {
            html: breadcrumbs(&items),
            css: breadcrumbs_css(),
            js: breadcrumbs_js(),
        }
    }

    fn sample_cards(include_draft: bool) -> Bundle {
        let mut html = String::new();
        html.push_str(&card(
            "example-post",
            "",
            "https://picsum.photos/seed/greatlittle/800/450",
            "Abstract placeholder cover",
            "lazy",
            "",
            "2026-04-19",
            "April 19, 2026",
            "A Great Little Example Post",
            "A short description that gives a taste of what the article is about.",
        ));
        html.push_str(&card(
            "another-post",
            "",
            "",
            "",
            "lazy",
            "",
            "2026-03-02",
            "March 2, 2026",
            "Notes Without a Cover",
            "When there is no cover image the card collapses to just the text block.",
        ));
        if include_draft {
            html.push_str(&card(
                "draft-post",
                "draft",
                "https://picsum.photos/seed/draft/800/450",
                "Abstract placeholder cover",
                "lazy",
                "",
                "2026-04-20",
                "April 20, 2026",
                "An Unfinished Draft",
                "Drafts appear in the listing when previewing with drafts included.",
            ));
        }
        let news = Bundle {
            html: newsletter(),
            css: newsletter_css(),
            js: newsletter_js(),
        };
        Bundle {
            html,
            css: card_css(),
            js: card_js(),
        }
    }

    fn render(cards: Bundle, news: Bundle) -> Bundle {
        index(crumbs(), cards, news)
    }

    fn news() -> Bundle {
        Bundle {
            html: newsletter(),
            css: newsletter_css(),
            js: newsletter_js(),
        }
    }

    #[preview("Blog index/Default")]
    fn default() -> Bundle {
        render(sample_cards(false), news())
    }

    #[preview("Blog index/With draft")]
    fn with_draft() -> Bundle {
        render(sample_cards(true), news())
    }

    #[preview("Blog index/Empty")]
    fn empty() -> Bundle {
        render(
            Bundle {
                html: String::new(),
                css: card_css(),
                js: card_js(),
            },
            news(),
        )
    }
}
