pub mod template;

pub struct Bundle {
    pub html: String,
    pub css: String,
    pub js: String,
}

pub fn finalize(b: Bundle) -> String {
    let Bundle { mut html, css, js } = b;
    if !css.is_empty() {
        let tag = format!("<style>{css}</style>\n");
        if let Some(idx) = html.find("</head>") {
            html.insert_str(idx, &tag);
        } else {
            html.insert_str(0, &tag);
        }
    }
    if !js.is_empty() {
        let tag = format!("<script>{js}</script>\n");
        if let Some(idx) = html.find("</body>") {
            html.insert_str(idx, &tag);
        } else {
            html.push_str(&tag);
        }
    }
    minify(&html)
}

pub fn escape_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

pub fn json_escape(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for c in value.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '<' => out.push_str("\\u003c"),
            '>' => out.push_str("\\u003e"),
            '&' => out.push_str("\\u0026"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

fn minify(html: &str) -> String {
    let mut cfg = minify_html::Cfg::new();
    cfg.minify_css = true;
    cfg.minify_js = true;
    cfg.keep_html_and_head_opening_tags = true;
    cfg.keep_closing_tags = true;
    let bytes = minify_html::minify(html.as_bytes(), &cfg);
    String::from_utf8(bytes).expect("minify-html produced invalid utf-8")
}

#[cfg(test)]
mod tests {
    use super::{finalize, Bundle};

    #[test]
    fn injects_style_before_head_close() {
        let b = Bundle {
            html: "<!doctype html><html><head><title>t</title></head><body></body></html>".into(),
            css: "body{color:red}".into(),
            js: String::new(),
        };
        let out = finalize(b);
        assert!(out.contains("<style>body{color:red}</style></head>"));
    }

    #[test]
    fn injects_script_before_body_close() {
        let b = Bundle {
            html: "<!doctype html><html><head></head><body><p>hi</p></body></html>".into(),
            css: String::new(),
            js: "console.log(1)".into(),
        };
        let out = finalize(b);
        assert!(out.contains("<script>console.log(1)</script></body>"));
    }

    #[test]
    fn empty_css_and_js_leaves_html_untouched() {
        let html = "<!doctype html><html><head></head><body></body></html>".to_string();
        let b = Bundle { html: html.clone(), css: String::new(), js: String::new() };
        assert_eq!(finalize(b), "<!doctype html><html><head></head><body></body></html>");
    }
}
