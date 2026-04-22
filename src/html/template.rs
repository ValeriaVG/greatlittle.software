use std::collections::HashMap;

use super::{escape_html, json_escape};

pub enum Part {
    Text(String),
    Field(String, String),
    Scalar(String),
    Slot(String),
    If(Cond, Vec<Part>),
    For(String, String, Vec<Part>),
}

#[derive(Clone)]
pub enum Cond {
    Scalar(String),
    Field(String, String),
}

pub enum Value {
    Scalar(String),
    Group(HashMap<String, String>),
    Repeat(Vec<HashMap<String, String>>),
    Slot(String),
}

#[derive(Clone, Copy)]
pub enum Escape {
    Html,
    Json,
}

const IF_OPEN: &str = "<!--if ";
const IF_CLOSE: &str = "<!--/if-->";
const IF_OPEN_END: &str = "-->";
const FOR_OPEN: &str = "<!--for ";
const FOR_CLOSE: &str = "<!--/for-->";
const BLOCK_OPEN_END: &str = "-->";

pub fn parse_template(template: &str) -> Vec<Part> {
    let mut parts = Vec::new();
    let bytes = template.as_bytes();
    let mut i = 0;
    let mut text_start = 0;
    while i < bytes.len() {
        if bytes[i] == b'<' && starts_slot_tag(&template[i..]) {
            if text_start < i {
                parts.push(Part::Text(template[text_start..i].to_string()));
            }
            let tag_end = template[i..].find('>').expect("unclosed <slot> tag") + i;
            let open = &template[i..=tag_end];
            let name = extract_name_attr(open)
                .unwrap_or_else(|| panic!("<slot> missing `name` attribute"));
            let self_closing = open.trim_end_matches('>').trim_end().ends_with('/');
            let after = if self_closing {
                tag_end + 1
            } else {
                let rest = &template[tag_end + 1..];
                let close_rel = rest.find("</slot>").expect("unclosed <slot> element");
                tag_end + 1 + close_rel + "</slot>".len()
            };
            if !is_ident(&name) {
                panic!("invalid slot name `{name}`");
            }
            parts.push(Part::Slot(name));
            i = after;
            text_start = i;
            continue;
        }
        if template[i..].starts_with(FOR_OPEN) {
            let name_start = i + FOR_OPEN.len();
            let open_end_rel = template[name_start..]
                .find(BLOCK_OPEN_END)
                .unwrap_or_else(|| panic!("unclosed `<!--for ...` comment"));
            let decl = template[name_start..name_start + open_end_rel].trim();
            let (var, coll) = parse_for_decl(decl);
            let body_start = name_start + open_end_rel + BLOCK_OPEN_END.len();
            let (body_end, close_end) = find_for_close(template, body_start)
                .unwrap_or_else(|| panic!("missing `<!--/for-->` for `{var}`"));
            if text_start < i {
                parts.push(Part::Text(template[text_start..i].to_string()));
            }
            let body_parts = parse_template(&template[body_start..body_end]);
            parts.push(Part::For(var, coll, body_parts));
            i = close_end;
            text_start = i;
            continue;
        }
        if template[i..].starts_with(IF_OPEN) {
            let name_start = i + IF_OPEN.len();
            let open_end_rel = template[name_start..]
                .find(IF_OPEN_END)
                .unwrap_or_else(|| panic!("unclosed `<!--if ...` comment"));
            let name = template[name_start..name_start + open_end_rel].trim();
            let body_start = name_start + open_end_rel + IF_OPEN_END.len();
            let (body_end, close_end) = find_if_close(template, body_start)
                .unwrap_or_else(|| panic!("missing `<!--/if-->` for `{name}`"));
            if text_start < i {
                parts.push(Part::Text(template[text_start..i].to_string()));
            }
            let body_parts = parse_template(&template[body_start..body_end]);
            parts.push(Part::If(parse_cond(name), body_parts));
            i = close_end;
            text_start = i;
            continue;
        }
        if bytes[i] == b'{' {
            if let Some(rel) = template[i + 1..].find('}') {
                let end = i + 1 + rel;
                let ph = template[i + 1..end].trim();
                let placeholder = if let Some(dot) = ph.find('.') {
                    let g = ph[..dot].trim();
                    let f = ph[dot + 1..].trim();
                    if is_ident(g) && is_ident(f) {
                        Some(Part::Field(g.to_string(), f.to_string()))
                    } else {
                        None
                    }
                } else if is_ident(ph) {
                    Some(Part::Scalar(ph.to_string()))
                } else {
                    None
                };
                if let Some(part) = placeholder {
                    if text_start < i {
                        parts.push(Part::Text(template[text_start..i].to_string()));
                    }
                    parts.push(part);
                    i = end + 1;
                    text_start = i;
                    continue;
                }
            }
            i += 1;
        } else {
            i += 1;
        }
    }
    if text_start < bytes.len() {
        parts.push(Part::Text(template[text_start..].to_string()));
    }
    parts
}

fn find_if_close(template: &str, from: usize) -> Option<(usize, usize)> {
    let mut depth = 1usize;
    let mut i = from;
    while i < template.len() {
        if template[i..].starts_with(IF_CLOSE) {
            depth -= 1;
            if depth == 0 {
                return Some((i, i + IF_CLOSE.len()));
            }
            i += IF_CLOSE.len();
            continue;
        }
        if template[i..].starts_with(IF_OPEN) {
            let rel = template[i + IF_OPEN.len()..].find(IF_OPEN_END)?;
            depth += 1;
            i += IF_OPEN.len() + rel + IF_OPEN_END.len();
            continue;
        }
        i += 1;
    }
    None
}

fn find_for_close(template: &str, from: usize) -> Option<(usize, usize)> {
    let mut depth = 1usize;
    let mut i = from;
    while i < template.len() {
        if template[i..].starts_with(FOR_CLOSE) {
            depth -= 1;
            if depth == 0 {
                return Some((i, i + FOR_CLOSE.len()));
            }
            i += FOR_CLOSE.len();
            continue;
        }
        if template[i..].starts_with(FOR_OPEN) {
            let rel = template[i + FOR_OPEN.len()..].find(BLOCK_OPEN_END)?;
            depth += 1;
            i += FOR_OPEN.len() + rel + BLOCK_OPEN_END.len();
            continue;
        }
        i += 1;
    }
    None
}

fn parse_for_decl(s: &str) -> (String, String) {
    let mut it = s.split_whitespace();
    let var = it.next().unwrap_or_else(|| panic!("`<!--for-->` missing variable name"));
    let kw = it.next().unwrap_or_else(|| panic!("`<!--for-->` missing `in`"));
    if kw != "in" { panic!("`<!--for {var} ...-->`: expected `in`, got `{kw}`"); }
    let coll = it.next().unwrap_or_else(|| panic!("`<!--for-->` missing collection"));
    if it.next().is_some() { panic!("`<!--for {var} in {coll} ...-->`: unexpected trailing tokens"); }
    if !is_ident(var) { panic!("invalid loop variable `{var}`"); }
    if !is_ident(coll) { panic!("invalid loop collection `{coll}`"); }
    (var.to_string(), coll.to_string())
}

fn parse_cond(name: &str) -> Cond {
    if let Some(dot) = name.find('.') {
        let g = name[..dot].trim();
        let f = name[dot + 1..].trim();
        if is_ident(g) && is_ident(f) {
            return Cond::Field(g.to_string(), f.to_string());
        }
        panic!("invalid `if` condition `{name}`");
    }
    if !is_ident(name) {
        panic!("invalid `if` condition `{name}`");
    }
    Cond::Scalar(name.to_string())
}

fn starts_slot_tag(s: &str) -> bool {
    let b = s.as_bytes();
    if !s.starts_with("<slot") { return false; }
    matches!(b.get(5), Some(&c) if c == b' ' || c == b'\t' || c == b'\n' || c == b'\r' || c == b'/' || c == b'>')
}

fn extract_name_attr(tag: &str) -> Option<String> {
    let idx = tag.find("name")?;
    let rest = tag[idx + 4..].trim_start();
    let rest = rest.strip_prefix('=')?.trim_start();
    let quote = rest.chars().next()?;
    if quote != '"' && quote != '\'' { return None; }
    let rest = &rest[1..];
    let end = rest.find(quote)?;
    Some(rest[..end].to_string())
}

fn is_ident(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

pub fn render(parts: &[Part], vars: &mut HashMap<String, Value>, esc: Escape) -> String {
    let mut out = String::new();
    render_into(&mut out, parts, vars, esc);
    out
}

fn render_into(out: &mut String, parts: &[Part], vars: &mut HashMap<String, Value>, esc: Escape) {
    for part in parts {
        match part {
            Part::Text(t) => out.push_str(t),
            Part::Scalar(name) => {
                let s = match vars.get(name) {
                    Some(Value::Scalar(v)) => v.as_str(),
                    Some(Value::Slot(v)) => v.as_str(),
                    _ => "",
                };
                out.push_str(&escape(s, esc));
            }
            Part::Field(g, f) => {
                let s = match vars.get(g) {
                    Some(Value::Group(m)) => m.get(f).map(String::as_str).unwrap_or(""),
                    _ => "",
                };
                out.push_str(&escape(s, esc));
            }
            Part::Slot(name) => {
                if let Some(Value::Slot(v)) = vars.get(name) {
                    out.push_str(v);
                }
            }
            Part::If(cond, body) => {
                let truthy = match cond {
                    Cond::Scalar(s) => match vars.get(s) {
                        Some(Value::Scalar(v)) => !v.is_empty(),
                        Some(Value::Slot(v)) => !v.is_empty(),
                        Some(Value::Group(_)) => true,
                        Some(Value::Repeat(r)) => !r.is_empty(),
                        None => false,
                    },
                    Cond::Field(g, f) => match vars.get(g) {
                        Some(Value::Group(m)) => m.get(f).map(|v| !v.is_empty()).unwrap_or(false),
                        _ => false,
                    },
                };
                if truthy {
                    render_into(out, body, vars, esc);
                }
            }
            Part::For(var, coll, body) => {
                let rows: Vec<HashMap<String, String>> = match vars.get(coll) {
                    Some(Value::Repeat(r)) => r.clone(),
                    _ => Vec::new(),
                };
                for row in rows {
                    let prev = vars.insert(var.clone(), Value::Group(row));
                    render_into(out, body, vars, esc);
                    match prev {
                        Some(p) => { vars.insert(var.clone(), p); }
                        None => { vars.remove(var); }
                    }
                }
            }
        }
    }
}

fn escape(s: &str, esc: Escape) -> String {
    match esc {
        Escape::Html => escape_html(s),
        Escape::Json => json_escape(s),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scalar(s: &str) -> Value { Value::Scalar(s.to_string()) }

    fn render_html(src: &str, vars: &mut HashMap<String, Value>) -> String {
        render(&parse_template(src), vars, Escape::Html)
    }

    #[test]
    fn scalar_interpolation_escapes_html() {
        let mut vars = HashMap::new();
        vars.insert("name".into(), scalar("<b>&\"x\"</b>"));
        assert_eq!(render_html("hi {name}!", &mut vars), "hi &lt;b&gt;&amp;&quot;x&quot;&lt;/b&gt;!");
    }

    #[test]
    fn missing_scalar_renders_empty() {
        let mut vars = HashMap::new();
        assert_eq!(render_html("[{missing}]", &mut vars), "[]");
    }

    #[test]
    fn field_access_from_group() {
        let mut vars = HashMap::new();
        let mut g = HashMap::new();
        g.insert("src".into(), "/a.png".into());
        g.insert("alt".into(), "A".into());
        vars.insert("img".into(), Value::Group(g));
        assert_eq!(render_html("<img src=\"{img.src}\" alt=\"{img.alt}\">", &mut vars),
            "<img src=\"/a.png\" alt=\"A\">");
    }

    #[test]
    fn slot_inserts_raw_html_without_escaping() {
        let mut vars = HashMap::new();
        vars.insert("body".into(), Value::Slot("<p>hi</p>".into()));
        assert_eq!(render_html("<main><slot name=\"body\"></slot></main>", &mut vars),
            "<main><p>hi</p></main>");
    }

    #[test]
    fn if_block_included_when_scalar_nonempty() {
        let mut vars = HashMap::new();
        vars.insert("flag".into(), scalar("yes"));
        assert_eq!(render_html("<!--if flag-->on<!--/if-->", &mut vars), "on");
    }

    #[test]
    fn if_block_omitted_when_scalar_empty() {
        let mut vars = HashMap::new();
        vars.insert("flag".into(), scalar(""));
        assert_eq!(render_html("<!--if flag-->on<!--/if-->", &mut vars), "");
    }

    #[test]
    fn if_block_omitted_when_var_missing() {
        let mut vars = HashMap::new();
        assert_eq!(render_html("<!--if absent-->x<!--/if-->", &mut vars), "");
    }

    #[test]
    fn if_field_tests_group_field_emptiness() {
        let mut vars = HashMap::new();
        let mut g = HashMap::new();
        g.insert("title".into(), "T".into());
        g.insert("subtitle".into(), "".into());
        vars.insert("post".into(), Value::Group(g));
        assert_eq!(render_html("<!--if post.title-->t<!--/if-->|<!--if post.subtitle-->s<!--/if-->", &mut vars), "t|");
    }

    #[test]
    fn for_loop_iterates_rows() {
        let mut vars = HashMap::new();
        let rows: Vec<HashMap<String, String>> = ["a", "b", "c"].iter().map(|x| {
            let mut m = HashMap::new();
            m.insert("label".into(), (*x).into());
            m
        }).collect();
        vars.insert("items".into(), Value::Repeat(rows));
        let out = render_html("<ul><!--for item in items--><li>{item.label}</li><!--/for--></ul>", &mut vars);
        assert_eq!(out, "<ul><li>a</li><li>b</li><li>c</li></ul>");
    }

    #[test]
    fn for_loop_with_empty_collection_renders_nothing_inside() {
        let mut vars = HashMap::new();
        vars.insert("items".into(), Value::Repeat(Vec::new()));
        assert_eq!(render_html("[<!--for i in items-->{i.x}<!--/for-->]", &mut vars), "[]");
    }

    #[test]
    fn for_loop_body_can_see_outer_scalars() {
        let mut vars = HashMap::new();
        vars.insert("sep".into(), scalar("-"));
        let rows: Vec<HashMap<String, String>> = ["a", "b"].iter().map(|x| {
            let mut m = HashMap::new();
            m.insert("v".into(), (*x).into());
            m
        }).collect();
        vars.insert("rows".into(), Value::Repeat(rows));
        let out = render_html("<!--for r in rows-->{r.v}{sep}<!--/for-->", &mut vars);
        assert_eq!(out, "a-b-");
    }

    #[test]
    fn for_loop_restores_shadowed_binding() {
        let mut vars = HashMap::new();
        vars.insert("r".into(), scalar("outer"));
        let rows = vec![{ let mut m = HashMap::new(); m.insert("v".into(), "inner".into()); m }];
        vars.insert("rows".into(), Value::Repeat(rows));
        let out = render_html("<!--for r in rows-->{r.v}<!--/for-->|{r}", &mut vars);
        assert_eq!(out, "inner|outer");
    }

    #[test]
    fn nested_if_inside_for() {
        let mut vars = HashMap::new();
        let rows: Vec<HashMap<String, String>> = [("a", "1"), ("b", ""), ("c", "3")].iter().map(|(k, v)| {
            let mut m = HashMap::new();
            m.insert("k".into(), (*k).into());
            m.insert("v".into(), (*v).into());
            m
        }).collect();
        vars.insert("rows".into(), Value::Repeat(rows));
        let out = render_html("<!--for r in rows--><!--if r.v-->{r.k}={r.v};<!--/if--><!--/for-->", &mut vars);
        assert_eq!(out, "a=1;c=3;");
    }

    #[test]
    fn json_escape_mode_escapes_quotes_and_angles() {
        let mut vars = HashMap::new();
        vars.insert("name".into(), scalar("a\"b<c>"));
        let out = render(&parse_template("{\"name\": \"{name}\"}"), &mut vars, Escape::Json);
        assert_eq!(out, "{\"name\": \"a\\\"b\\u003cc\\u003e\"}");
    }

    #[test]
    fn text_between_placeholders_is_preserved_verbatim() {
        let mut vars = HashMap::new();
        vars.insert("a".into(), scalar("X"));
        vars.insert("b".into(), scalar("Y"));
        assert_eq!(render_html("  {a}\n<hr/>\n{b}  ", &mut vars), "  X\n<hr/>\nY  ");
    }

    #[test]
    fn unclosed_placeholder_is_left_as_literal() {
        let mut vars = HashMap::new();
        assert_eq!(render_html("pre { not a placeholder", &mut vars), "pre { not a placeholder");
    }
}
