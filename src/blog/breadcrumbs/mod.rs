#[cfg(feature = "preview")]
mod previews {
    use macros::preview;

    use crate::html::Bundle;

    use super::super::{BreadcrumbsItem, breadcrumbs, breadcrumbs_css, breadcrumbs_js};

    fn link(href: &str, label: &str) -> BreadcrumbsItem {
        BreadcrumbsItem { href: href.into(), label: label.into(), current: String::new() }
    }

    fn current(label: &str) -> BreadcrumbsItem {
        BreadcrumbsItem { href: String::new(), label: label.into(), current: "page".into() }
    }

    fn bundle(items: &[BreadcrumbsItem]) -> Bundle {
        Bundle { html: breadcrumbs(items), css: breadcrumbs_css(), js: breadcrumbs_js() }
    }

    #[preview("Breadcrumbs/Two levels")]
    fn two_levels() -> Bundle {
        bundle(&[link("/", "Home"), current("Blog")])
    }

    #[preview("Breadcrumbs/Three levels")]
    fn three_levels() -> Bundle {
        bundle(&[
            link("/", "Home"),
            link("/blog/", "Blog"),
            current("A Great Little Example Post"),
        ])
    }
}
