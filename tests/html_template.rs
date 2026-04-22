use greatlittle_software::html::{template, Fragment};
use macros::html_template;

html_template!(render_example, "tests/fixtures/example.html");
html_template!(render_scalars, "tests/fixtures/scalars_only.html");
html_template!(render_repeated, "tests/fixtures/repeated.html");

#[test]
fn renders_example_with_group_and_scalars() {
    let image = RenderExampleImage {
        src: "/img/hello.png".into(),
        alt_text: "Hello".into(),
    };
    let out = render_example(&image, "Welcome", "Body text.").html;
    let expected = "<section>\n    <img src=\"/img/hello.png\" alt=\"Hello\" />\n    <h1>Welcome</h1>\n    <p>Body text.</p>\n</section>\n";
    assert_eq!(out, expected);
}

#[test]
fn preserves_static_markup_verbatim() {
    let image = RenderExampleImage { src: "a".into(), alt_text: "b".into() };
    let out = render_example(&image, "t", "c").html;
    assert!(out.starts_with("<section>"));
    assert!(out.trim_end().ends_with("</section>"));
}

#[test]
fn escapes_special_characters_in_template_literals() {
    let image = RenderExampleImage { src: "x".into(), alt_text: "y".into() };
    let out = render_example(&image, "t", "c").html;
    assert!(out.contains("src=\"x\""));
    assert!(out.contains("alt=\"y\""));
}

#[test]
fn substitutes_values_not_placeholder_text() {
    let image = RenderExampleImage { src: "S".into(), alt_text: "A".into() };
    let out = render_example(&image, "T", "C").html;
    assert!(!out.contains("{title}"));
    assert!(!out.contains("{image.src}"));
    assert!(!out.contains("{image.alt_text}"));
    assert!(!out.contains("{content}"));
}

#[test]
fn scalars_only_template() {
    let out = render_scalars("Hi", "there").html;
    assert_eq!(out, "<h1>Hi</h1>\n<p>there</p>\n");
}

#[test]
fn repeated_placeholders_share_one_parameter() {
    let link = RenderRepeatedLink {
        href: "/x".into(),
        label: "X".into(),
    };
    let out = render_repeated(&link).html;
    assert_eq!(
        out,
        "<a href=\"/x\">X</a> and again <a href=\"/x\">X</a>\n"
    );
}

#[test]
fn empty_values_render_as_empty_strings() {
    let image = RenderExampleImage { src: String::new(), alt_text: String::new() };
    let out = render_example(&image, "", "").html;
    assert!(out.contains("src=\"\""));
    assert!(out.contains("<h1></h1>"));
    assert!(out.contains("<p></p>"));
}

#[test]
fn for_loop_renders_each_item() {
    html_template!(render_looped, "tests/fixtures/looped.html");
    let items = vec![
        RenderLoopedItem { href: "/".into(), label: "Home".into(), current: String::new() },
        RenderLoopedItem { href: "/blog/".into(), label: "Blog".into(), current: String::new() },
        RenderLoopedItem { href: String::new(), label: "Post".into(), current: "page".into() },
    ];
    let out = render_looped(&items).html;
    assert!(out.contains("<a href=\"/\">Home</a>"));
    assert!(out.contains("<a href=\"/blog/\">Blog</a>"));
    assert!(out.contains("<span>Post</span>"));
    assert!(!out.contains("<a href=\"\">"));
    assert_eq!(out.matches("<li>").count(), 3);
}

#[test]
fn slot_fragments_bubble_css_and_js() {
    html_template!(render_slotted, "tests/fixtures/slotted.html");
    let child = Fragment {
        html: "<p>hi</p>".into(),
        css: "p{color:red}".into(),
        js: "x=1".into(),
    };
    let out = render_slotted(child);
    assert!(out.html.contains("<main><p>hi</p></main>"));
    assert!(out.css.contains("p{color:red}"));
    assert!(out.js.contains("x=1"));
}
