use macros::html_template;

use crate::html::Fragment;

html_template!(intro, "src/home/intro");
html_template!(layout, "src/home/layout");

pub fn render() -> Fragment {
    const TITLE:&str= "Great Little Software";
    layout(
        TITLE,
        intro(
            &(TITLE.to_owned()+".")
        )
    )
}
