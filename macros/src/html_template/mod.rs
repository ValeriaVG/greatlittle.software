use proc_macro::{TokenStream, TokenTree};
use std::path::PathBuf;

mod codegen;
mod parse;

use parse::{parse_template, Cond, Part};

pub(crate) fn expand(input: TokenStream) -> TokenStream {
    let (fn_name, path) = parse_args(input);

    let manifest = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR not set");
    let base = PathBuf::from(&manifest).join(&path);
    let html_path = base.with_extension("html");
    let css_path = base.with_extension("css");
    let js_path = base.with_extension("js");
    let ld_path = PathBuf::from(format!("{}.ld.json", base.display()));

    let template_src = std::fs::read_to_string(&html_path)
        .unwrap_or_else(|e| panic!("html_template!: failed to read {}: {e}", html_path.display()));
    let ld_src = std::fs::read_to_string(&ld_path).ok();

    let html_parts = parse_template(&template_src);
    let ld_parts = ld_src.as_deref().map(parse_template);

    let sig = collect_signature(&html_parts, ld_parts.as_deref());

    let mut tracker = String::new();
    let mut tracked = vec![&html_path, &css_path, &js_path];
    if ld_src.is_some() {
        tracked.push(&ld_path);
    }
    for path in tracked {
        if path.exists() {
            tracker.push_str(&format!(
                "const _: &[u8] = include_bytes!({:?});\n",
                path.to_string_lossy(),
            ));
        }
    }
    let html_path_str = html_path.to_string_lossy().to_string();
    let css_path_str = css_path.to_string_lossy().to_string();
    let js_path_str = js_path.to_string_lossy().to_string();
    let ld_path_str = ld_src.as_ref().map(|_| ld_path.to_string_lossy().to_string());
    let body = codegen::render(
        &fn_name,
        &sig,
        &html_path_str,
        &css_path_str,
        &js_path_str,
        ld_path_str.as_deref(),
    );
    format!("{tracker}{body}")
        .parse()
        .expect("html_template!: generated code failed to parse")
}

pub(crate) struct Signature {
    pub groups: Vec<(String, Vec<String>)>,
    pub slots: Vec<String>,
    pub repeats: Vec<Repeat>,
    pub root_order: Vec<(String, Kind)>,
}

pub(crate) struct Repeat {
    pub var: String,
    pub coll: String,
    pub fields: Vec<String>,
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum Kind { Scalar, Group, Slot, Repeat }

fn collect_signature(html_parts: &[Part], ld_parts: Option<&[Part]>) -> Signature {
    let mut sig = Signature { groups: Vec::new(), slots: Vec::new(), repeats: Vec::new(), root_order: Vec::new() };
    let mut scalars: Vec<String> = Vec::new();
    let mut scope: Vec<(String, usize)> = Vec::new();
    walk(html_parts, &mut sig, &mut scalars, &mut scope);
    if let Some(parts) = ld_parts {
        walk(parts, &mut sig, &mut scalars, &mut scope);
    }
    sig
}

fn walk(parts: &[Part], sig: &mut Signature, scalars: &mut Vec<String>, scope: &mut Vec<(String, usize)>) {
    for part in parts {
        match part {
            Part::Field(g, f) => register_field_or_repeat(sig, scalars, scope, g, f),
            Part::Scalar(s) => {
                if scope.iter().any(|(v, _)| v == s) {
                    panic!("`{s}` is a loop variable; use `{s}.field` to access fields");
                }
                register_scalar(sig, scalars, s);
            }
            Part::Slot(s) => register_slot(sig, scalars, s),
            Part::If(cond, body) => {
                match cond {
                    Cond::Scalar(s) => {
                        if !scope.iter().any(|(v, _)| v == s) {
                            register_scalar(sig, scalars, s);
                        }
                    }
                    Cond::Field(g, f) => register_field_or_repeat(sig, scalars, scope, g, f),
                }
                walk(body, sig, scalars, scope);
            }
            Part::For(var, coll, body) => {
                if sig.groups.iter().any(|(n, _)| n == coll) { panic!("`{coll}` used as both group and collection"); }
                if scalars.iter().any(|n| n == coll) { panic!("`{coll}` used as both scalar and collection"); }
                if sig.slots.iter().any(|n| n == coll) { panic!("`{coll}` used as both slot and collection"); }
                let idx = if let Some(pos) = sig.repeats.iter().position(|r| r.coll == *coll) {
                    if sig.repeats[pos].var != *var {
                        panic!("collection `{coll}` already bound to loop variable `{}`", sig.repeats[pos].var);
                    }
                    pos
                } else {
                    sig.repeats.push(Repeat { var: var.clone(), coll: coll.clone(), fields: Vec::new() });
                    sig.root_order.push((coll.clone(), Kind::Repeat));
                    sig.repeats.len() - 1
                };
                scope.push((var.clone(), idx));
                walk(body, sig, scalars, scope);
                scope.pop();
            }
            Part::Text(_) => {}
        }
    }
}

fn register_field_or_repeat(sig: &mut Signature, scalars: &mut Vec<String>, scope: &[(String, usize)], g: &str, f: &str) {
    if let Some(&(_, idx)) = scope.iter().rev().find(|(v, _)| v == g) {
        let fields = &mut sig.repeats[idx].fields;
        if !fields.iter().any(|n| n == f) { fields.push(f.to_string()); }
    } else {
        register_field(sig, scalars, g, f);
    }
}

fn register_field(sig: &mut Signature, scalars: &[String], g: &str, f: &str) {
    if scalars.iter().any(|n| n == g) { panic!("`{g}` used as both scalar and group"); }
    if sig.slots.iter().any(|n| n == g) { panic!("`{g}` used as both slot and group"); }
    if sig.repeats.iter().any(|r| r.coll == g) { panic!("`{g}` used as both collection and group"); }
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
    if sig.repeats.iter().any(|r| r.coll == s) { panic!("`{s}` used as both collection and scalar"); }
    if !scalars.iter().any(|n| n == s) { scalars.push(s.to_string()); }
    if !sig.root_order.iter().any(|(n, _)| n == s) {
        sig.root_order.push((s.to_string(), Kind::Scalar));
    }
}

fn register_slot(sig: &mut Signature, scalars: &[String], s: &str) {
    if sig.groups.iter().any(|(n, _)| n == s) { panic!("`{s}` used as both group and slot"); }
    if scalars.iter().any(|n| n == s) { panic!("`{s}` used as both scalar and slot"); }
    if sig.repeats.iter().any(|r| r.coll == s) { panic!("`{s}` used as both collection and slot"); }
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
