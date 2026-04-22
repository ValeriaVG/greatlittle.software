pub(crate) enum Part {
    Text(String),
    Field(String, String),
    Scalar(String),
    Slot(String),
    If(Cond, Vec<Part>),
}

#[derive(Clone)]
pub(crate) enum Cond {
    Scalar(String),
    Field(String, String),
}

const IF_OPEN: &str = "<!--if ";
const IF_CLOSE: &str = "<!--/if-->";
const IF_OPEN_END: &str = "-->";

pub(crate) fn parse_template(template: &str) -> Vec<Part> {
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
