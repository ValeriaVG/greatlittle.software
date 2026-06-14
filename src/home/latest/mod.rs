#[cfg(feature = "preview")]
mod previews {
    use macros::preview;

    use crate::html::Bundle;

    use super::super::latest;
    use crate::blog::{card, card_css, card_js};

    #[preview("Home/Latest")]
    fn default() -> Bundle {
        let mut featured_html = String::new();
        featured_html.push_str(&card(
            "example-post",
            "",
            "https://picsum.photos/seed/greatlittle/800/450",
            "Abstract placeholder cover",
            "",
            "high",
            "2026-04-19",
            "April 19, 2026",
            "A Great Little Example Post",
            "A short description that gives a taste of what the article is about.",
        ));
        let featured = Bundle {
            html: featured_html,
            css: card_css(),
            js: card_js(),
        };

        let mut stories_html = String::new();
        stories_html.push_str(&card(
            "papra",
            "",
            "https://picsum.photos/seed/papra/800/450",
            "Papra screenshot",
            "lazy",
            "",
            "2026-05-10",
            "May 10, 2026",
            "Papra: Open Source Document Management",
            "Taming document chaos with an open-source tool.",
        ));
        stories_html.push_str(&card(
            "rackula",
            "",
            "https://picsum.photos/seed/rackula/800/450",
            "Rackula screenshot",
            "lazy",
            "",
            "2026-04-28",
            "April 28, 2026",
            "Rackula: Server Rack Planner",
            "Drag and drop rack visualizer for homelabbers.",
        ));
        let stories = Bundle {
            html: stories_html,
            css: String::new(),
            js: String::new(),
        };

        let mut thoughts_html = String::new();
        thoughts_html.push_str(&card(
            "evilcorp",
            "",
            "",
            "",
            "lazy",
            "",
            "2026-05-30",
            "May 30, 2026",
            "EvilCorp Syndrome",
            "An essay on sales, marketing and better ways to do it.",
        ));
        let thoughts = Bundle {
            html: thoughts_html,
            css: String::new(),
            js: String::new(),
        };

        latest("Featured story", featured, "yes", stories, "yes", thoughts)
    }
}
