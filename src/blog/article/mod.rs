#[cfg(feature = "preview")]
mod previews {
    use macros::preview;

    use crate::html::Bundle;

    use super::super::{
        ArticleAction, ArticleArticle, ArticleCover, ArticleProduct, BreadcrumbsItem, article,
        breadcrumbs, breadcrumbs_css, breadcrumbs_js, card, card_css, card_js, newsletter,
        newsletter_css, newsletter_js, related,
    };

    const BODY_HTML: &str = "<p>A short opening paragraph that sets up the piece and gives a flavour of the writing voice.</p>\n<h2>A section heading</h2>\n<p>Body copy with <a href=\"#\">a link</a>, <em>emphasis</em> and <strong>strong</strong> text so the prose styles get a workout.</p>\n<ul><li>First bullet</li><li>Second bullet</li><li>Third bullet</li></ul>\n<blockquote><p>A pull quote to check blockquote styles.</p></blockquote>\n<pre><code>fn main() {\n    println!(\"hi\");\n}\n</code></pre>\n";

    fn crumbs() -> Bundle {
        let items = [
            BreadcrumbsItem { href: "/".into(), label: "Home".into(), current: String::new() },
            BreadcrumbsItem { href: "/blog/".into(), label: "Blog".into(), current: String::new() },
            BreadcrumbsItem {
                href: String::new(),
                label: "A Great Little Example Post".into(),
                current: "page".into(),
            },
        ];
        Bundle { html: breadcrumbs(&items), css: breadcrumbs_css(), js: breadcrumbs_js() }
    }

    fn body() -> Bundle {
        Bundle { html: BODY_HTML.into(), css: String::new(), js: String::new() }
    }

    fn news() -> Bundle {
        Bundle { html: newsletter(), css: newsletter_css(), js: newsletter_js() }
    }

    fn related_cards() -> Bundle {
        let html = card(
            "another-post",
            "",
            "https://picsum.photos/seed/greatlittle/800/450",
            "Another placeholder cover",
            "2026-03-02",
            "March 2, 2026",
            "Another Great Little Article",
            "A short description of another article.",
        );
        Bundle { html, css: card_css(), js: card_js() }
    }

    fn rel() -> Bundle {
        related("yes", related_cards())
    }

    fn empty_related() -> Bundle {
        related("", Bundle { html: String::new(), css: String::new(), js: String::new() })
    }

    fn sample_article() -> ArticleArticle {
        ArticleArticle {
            title: "A Great Little Example Post".into(),
            url: "https://greatlittle.software/blog/example-post/".into(),
            description: "A short description that gives a taste of what the article is about.".into(),
            author: "Great Little Software".into(),
        }
    }

    fn empty_product() -> ArticleProduct {
        ArticleProduct { name: String::new(), cover: String::new(), blurb: String::new() }
    }

    fn sample_product() -> ArticleProduct {
        ArticleProduct {
            name: "psikat".into(),
            cover: "https://picsum.photos/seed/psikat/640/360".into(),
            blurb: "A tracker-like DAW in your browser.".into(),
        }
    }

    fn sample_actions() -> Vec<ArticleAction> {
        vec![
            ArticleAction { label: "Try it".into(), url: "https://psikat.com".into() },
            ArticleAction { label: "Join Discord".into(), url: "https://discord.gg/example".into() },
        ]
    }

    #[preview("Article/Default/Full")]
    fn full() -> Bundle {
        let cover = ArticleCover {
            src: "https://picsum.photos/seed/greatlittle/1200/600".into(),
            alt: "Abstract placeholder cover".into(),
            url: "https://picsum.photos/seed/greatlittle/1200/600".into(),
        };
        article(
            crumbs(),
            "",
            "2026-04-19",
            "April 19, 2026",
            &sample_article(),
            &cover,
            body(),
            &empty_product(),
            &[],
            rel(),
            news(),
            "2026-04-19",
            "indie software, writing",
        )
    }

    #[preview("Article/Default/No cover")]
    fn no_cover() -> Bundle {
        let cover = ArticleCover { src: String::new(), alt: String::new(), url: String::new() };
        article(
            crumbs(),
            "",
            "2026-03-02",
            "March 2, 2026",
            &sample_article(),
            &cover,
            body(),
            &empty_product(),
            &[],
            empty_related(),
            news(),
            "2026-03-02",
            "indie software",
        )
    }

    #[preview("Article/Default/With product")]
    fn with_product() -> Bundle {
        let cover = ArticleCover {
            src: "https://picsum.photos/seed/greatlittle/1200/600".into(),
            alt: "Abstract placeholder cover".into(),
            url: "https://picsum.photos/seed/greatlittle/1200/600".into(),
        };
        article(
            crumbs(),
            "",
            "2026-04-19",
            "April 19, 2026",
            &sample_article(),
            &cover,
            body(),
            &sample_product(),
            &sample_actions(),
            rel(),
            news(),
            "2026-04-19",
            "indie software, writing",
        )
    }

    #[preview("Article/Draft/Full")]
    fn draft_full() -> Bundle {
        let cover = ArticleCover {
            src: "https://picsum.photos/seed/greatlittle/1200/600".into(),
            alt: "Abstract placeholder cover".into(),
            url: "https://picsum.photos/seed/greatlittle/1200/600".into(),
        };
        article(
            crumbs(),
            "draft",
            "2026-04-19",
            "April 19, 2026",
            &sample_article(),
            &cover,
            body(),
            &empty_product(),
            &[],
            empty_related(),
            news(),
            "2026-04-19",
            "indie software, writing",
        )
    }
}
