use super::{Kind, Part, Signature};

pub(crate) fn render(
    fn_name: &str,
    sig: &Signature,
    parts: &[Part],
    css: &str,
    js: &str,
) -> String {
    let fn_pascal = to_pascal(fn_name);

    let mut out = String::new();
    for (name, fields) in &sig.groups {
        let struct_name = format!("{fn_pascal}{}", to_pascal(name));
        out.push_str(&format!("pub struct {struct_name} {{\n"));
        for f in fields {
            out.push_str(&format!("    pub {f}: String,\n"));
        }
        out.push_str("}\n");
    }

    let params: Vec<String> = sig.root_order.iter().map(|(n, k)| match k {
        Kind::Group => format!("{n}: &{}{}", fn_pascal, to_pascal(n)),
        Kind::Scalar => format!("{n}: &str"),
        Kind::Slot => format!("{n}: Fragment"),
    }).collect();

    out.push_str(&format!("pub fn {fn_name}({}) -> Fragment {{\n", params.join(", ")));
    out.push_str(&format!("    let mut __css = String::from({});\n", rust_str_lit(css)));
    out.push_str(&format!("    let mut __js = String::from({});\n", rust_str_lit(js)));
    for slot in &sig.slots {
        out.push_str(&format!("    if !{slot}.css.is_empty() {{ if !__css.is_empty() {{ __css.push('\\n'); }} __css.push_str(&{slot}.css); }}\n"));
        out.push_str(&format!("    if !{slot}.js.is_empty() {{ if !__js.is_empty() {{ __js.push('\\n'); }} __js.push_str(&{slot}.js); }}\n"));
    }
    out.push_str("    let mut __html = String::new();\n");
    for part in parts {
        match part {
            Part::Text(t) => {
                out.push_str(&format!("    __html.push_str({});\n", rust_str_lit(t)));
            }
            Part::Field(g, f) => {
                out.push_str(&format!("    __html.push_str(&{g}.{f});\n"));
            }
            Part::Scalar(s) => {
                out.push_str(&format!("    __html.push_str({s});\n"));
            }
            Part::Slot(s) => {
                out.push_str(&format!("    __html.push_str(&{s}.html);\n"));
            }
        }
    }
    out.push_str("    Fragment { html: __html, css: __css, js: __js }\n}\n");
    out
}

fn to_pascal(s: &str) -> String {
    let mut out = String::new();
    let mut up = true;
    for c in s.chars() {
        if c == '_' { up = true; continue; }
        if up { out.extend(c.to_uppercase()); up = false; }
        else { out.push(c); }
    }
    out
}

fn rust_str_lit(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{{{:x}}}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
