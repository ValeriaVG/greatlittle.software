use proc_macro::{TokenStream, TokenTree};
use std::path::PathBuf;

mod parse;
mod codegen;

use parse::{parse_template, Part};

pub(crate) fn expand(input: TokenStream) -> TokenStream {
    let (fn_name, path) = parse_args(input);

    let manifest = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR not set");
    let base = PathBuf::from(&manifest).join(&path);
    let html_path = base.with_extension("html");
    let css_path = base.with_extension("css");
    let js_path = base.with_extension("js");

    let template = std::fs::read_to_string(&html_path)
        .unwrap_or_else(|e| panic!("html_template!: failed to read {}: {e}", html_path.display()));
    let css_src = std::fs::read_to_string(&css_path).unwrap_or_default();
    let js_src = std::fs::read_to_string(&js_path).unwrap_or_default();

    let parts = parse_template(&template);

    let sig = collect_signature(&parts);

    let mut tracker = String::new();
    for path in [&html_path, &css_path, &js_path] {
        if path.exists() {
            tracker.push_str(&format!(
                "const _: &[u8] = include_bytes!({:?});\n",
                path.to_string_lossy(),
            ));
        }
    }
    let body = codegen::render(&fn_name, &sig, &parts, &css_src, &js_src);
    format!("{tracker}{body}")
        .parse()
        .expect("html_template!: generated code failed to parse")
}

pub(crate) struct Signature {
    pub groups: Vec<(String, Vec<String>)>,
    pub slots: Vec<String>,
    pub root_order: Vec<(String, Kind)>,
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum Kind { Scalar, Group, Slot }

fn collect_signature(parts: &[Part]) -> Signature {
    let mut groups: Vec<(String, Vec<String>)> = Vec::new();
    let mut scalars: Vec<String> = Vec::new();
    let mut slots: Vec<String> = Vec::new();
    let mut root_order: Vec<(String, Kind)> = Vec::new();
    for part in parts {
        match part {
            Part::Field(g, f) => {
                if scalars.contains(g) { panic!("`{g}` used as both scalar and group"); }
                if slots.contains(g) { panic!("`{g}` used as both slot and group"); }
                if let Some(e) = groups.iter_mut().find(|(n, _)| n == g) {
                    if !e.1.contains(f) { e.1.push(f.clone()); }
                } else {
                    groups.push((g.clone(), vec![f.clone()]));
                }
                if !root_order.iter().any(|(n, _)| n == g) {
                    root_order.push((g.clone(), Kind::Group));
                }
            }
            Part::Scalar(s) => {
                if groups.iter().any(|(n, _)| n == s) { panic!("`{s}` used as both group and scalar"); }
                if slots.contains(s) { panic!("`{s}` used as both slot and scalar"); }
                if !scalars.contains(s) { scalars.push(s.clone()); }
                if !root_order.iter().any(|(n, _)| n == s) {
                    root_order.push((s.clone(), Kind::Scalar));
                }
            }
            Part::Slot(s) => {
                if groups.iter().any(|(n, _)| n == s) { panic!("`{s}` used as both group and slot"); }
                if scalars.contains(s) { panic!("`{s}` used as both scalar and slot"); }
                if !slots.contains(s) { slots.push(s.clone()); }
                if !root_order.iter().any(|(n, _)| n == s) {
                    root_order.push((s.clone(), Kind::Slot));
                }
            }
            Part::Text(_) => {}
        }
    }
    Signature { groups, slots, root_order }
}

fn parse_args(input: TokenStream) -> (String, String) {
    let mut it = input.into_iter();
    let fn_name = match it.next() {
        Some(TokenTree::Ident(i)) => i.to_string(),
        _ => panic!("html_template!: expected function name identifier"),
    };
    match it.next() {
        Some(TokenTree::Punct(p)) if p.as_char() == ',' => {}
        _ => panic!("html_template!: expected `,` after function name"),
    }
    let path = match it.next() {
        Some(TokenTree::Literal(l)) => {
            let s = l.to_string();
            if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
                s[1..s.len() - 1].to_string()
            } else {
                panic!("html_template!: expected string literal path, got `{s}`");
            }
        }
        _ => panic!("html_template!: expected string literal path"),
    };
    (fn_name, path)
}
