pub struct Fragment {
    pub html: String,
    pub css: String,
    pub js: String,
}

pub fn finalize(f: Fragment) -> String {
    let Fragment { mut html, css, js } = f;
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
    html
}

#[cfg(test)]
mod tests {
    use super::{finalize, Fragment};

    #[test]
    fn injects_style_before_head_close() {
        let f = Fragment {
            html: "<!doctype html><html><head><title>t</title></head><body></body></html>".into(),
            css: "body{color:red}".into(),
            js: String::new(),
        };
        let out = finalize(f);
        assert!(out.contains("<style>body{color:red}</style>\n</head>"));
    }

    #[test]
    fn injects_script_before_body_close() {
        let f = Fragment {
            html: "<!doctype html><html><head></head><body><p>hi</p></body></html>".into(),
            css: String::new(),
            js: "console.log(1)".into(),
        };
        let out = finalize(f);
        assert!(out.contains("<script>console.log(1)</script>\n</body>"));
    }

    #[test]
    fn empty_css_and_js_leaves_html_untouched() {
        let html = "<!doctype html><html><head></head><body></body></html>".to_string();
        let f = Fragment { html: html.clone(), css: String::new(), js: String::new() };
        assert_eq!(finalize(f), html);
    }
}
