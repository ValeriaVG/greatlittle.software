pub(crate) enum Part {
    Text(String),
    Field(String, String),
    Scalar(String),
    Slot(String),
}

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
        if bytes[i] == b'{' {
            if text_start < i {
                parts.push(Part::Text(template[text_start..i].to_string()));
            }
            let rel = template[i + 1..].find('}').expect("unclosed placeholder");
            let end = i + 1 + rel;
            let ph = template[i + 1..end].trim();
            if let Some(dot) = ph.find('.') {
                let g = ph[..dot].trim().to_string();
                let f = ph[dot + 1..].trim().to_string();
                if !is_ident(&g) || !is_ident(&f) {
                    panic!("invalid placeholder `{{{ph}}}`");
                }
                parts.push(Part::Field(g, f));
            } else {
                if !is_ident(ph) {
                    panic!("invalid placeholder `{{{ph}}}`");
                }
                parts.push(Part::Scalar(ph.to_string()));
            }
            i = end + 1;
            text_start = i;
        } else {
            i += 1;
        }
    }
    if text_start < bytes.len() {
        parts.push(Part::Text(template[text_start..].to_string()));
    }
    parts
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
