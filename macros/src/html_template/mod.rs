use proc_macro::{TokenStream, TokenTree};
use std::path::PathBuf;

mod parse;
mod codegen;

use parse::{parse_template, Cond, Part};

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
    let mut sig = Signature { groups: Vec::new(), slots: Vec::new(), root_order: Vec::new() };
    let mut scalars: Vec<String> = Vec::new();
    walk(parts, &mut sig, &mut scalars);
    sig
}

fn walk(parts: &[Part], sig: &mut Signature, scalars: &mut Vec<String>) {
    for part in parts {
        match part {
            Part::Field(g, f) => register_field(sig, scalars, g, f),
            Part::Scalar(s) => register_scalar(sig, scalars, s),
            Part::Slot(s) => register_slot(sig, scalars, s),
            Part::If(cond, body) => {
                match cond {
                    Cond::Scalar(s) => register_scalar(sig, scalars, s),
                    Cond::Field(g, f) => register_field(sig, scalars, g, f),
                }
                walk(body, sig, scalars);
            }
            Part::Text(_) => {}
        }
    }
}

fn register_field(sig: &mut Signature, scalars: &[String], g: &str, f: &str) {
    if scalars.iter().any(|n| n == g) { panic!("`{g}` used as both scalar and group"); }
    if sig.slots.iter().any(|n| n == g) { panic!("`{g}` used as both slot and group"); }
    if let Some(e) = sig.groups.iter_mut().find(|(n, _)| n == g) {
        if !e.1.iter().any(|n| n == f) { e.1.push(f.to_string()); }
    } else {
        sig.groups.push((g.to_string(), vec![f.to_string()]));
    }
    if !sig.root_order.iter().any(|(n, _)| n == g) {
        sig.root_order.push((g.to_string(), Kind::Group));
    }
}

fn register_scalar(sig: &mut Signature, scalars: &mut Vec<String>, s: &str) {
    if sig.groups.iter().any(|(n, _)| n == s) { panic!("`{s}` used as both group and scalar"); }
    if sig.slots.iter().any(|n| n == s) { panic!("`{s}` used as both slot and scalar"); }
    if !scalars.iter().any(|n| n == s) { scalars.push(s.to_string()); }
    if !sig.root_order.iter().any(|(n, _)| n == s) {
        sig.root_order.push((s.to_string(), Kind::Scalar));
    }
}

fn register_slot(sig: &mut Signature, scalars: &[String], s: &str) {
    if sig.groups.iter().any(|(n, _)| n == s) { panic!("`{s}` used as both group and slot"); }
    if scalars.iter().any(|n| n == s) { panic!("`{s}` used as both scalar and slot"); }
    if !sig.slots.iter().any(|n| n == s) { sig.slots.push(s.to_string()); }
    if !sig.root_order.iter().any(|(n, _)| n == s) {
        sig.root_order.push((s.to_string(), Kind::Slot));
    }
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
