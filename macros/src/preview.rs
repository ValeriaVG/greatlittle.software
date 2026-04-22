use proc_macro::{TokenStream, TokenTree};

pub(crate) fn expand(attr: TokenStream, item: TokenStream) -> TokenStream {
    let path = parse_path(attr);
    let fn_name = parse_fn_name(item.clone());
    let path_escaped = escape(&path);
    let submit = format!(
        "#[cfg(feature = \"preview\")]\n\
         ::inventory::submit! {{\n\
             crate::preview::Preview {{\n\
                 path: \"{path_escaped}\",\n\
                 render: || crate::preview::IntoPreview::into_preview({fn_name}()),\n\
             }}\n\
         }}\n"
    );
    let out = format!("#[cfg(feature = \"preview\")]\n{item}\n{submit}");
    out.parse().expect("#[preview]: generated code failed to parse")
}

fn parse_path(attr: TokenStream) -> String {
    for tt in attr {
        if let TokenTree::Literal(l) = tt {
            let s = l.to_string();
            if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
                return s[1..s.len() - 1].to_string();
            }
            panic!("#[preview]: expected string literal, got `{s}`");
        }
    }
    panic!("#[preview]: expected a string literal path");
}

fn parse_fn_name(item: TokenStream) -> String {
    let mut it = item.into_iter();
    while let Some(tt) = it.next() {
        if let TokenTree::Ident(i) = &tt {
            if i.to_string() == "fn" {
                if let Some(TokenTree::Ident(n)) = it.next() {
                    return n.to_string();
                }
                panic!("#[preview]: missing function name after `fn`");
            }
        }
    }
    panic!("#[preview]: can only be applied to a function");
}

fn escape(s: &str) -> String {
    let mut o = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\\' => o.push_str("\\\\"),
            '"' => o.push_str("\\\""),
            _ => o.push(c),
        }
    }
    o
}
