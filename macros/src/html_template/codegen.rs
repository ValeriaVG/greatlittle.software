use super::{Kind, Signature};

pub(crate) fn render(
    fn_name: &str,
    sig: &Signature,
    html_path: &str,
    css_path: &str,
    js_path: &str,
    ld_path: Option<&str>,
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
    for r in &sig.repeats {
        let struct_name = format!("{fn_pascal}{}", to_pascal(&r.var));
        out.push_str(&format!("pub struct {struct_name} {{\n"));
        for f in &r.fields {
            out.push_str(&format!("    pub {f}: String,\n"));
        }
        out.push_str("}\n");
    }

    let params: Vec<String> = sig.root_order.iter().map(|(n, k)| match k {
        Kind::Group => format!("{n}: &{}{}", fn_pascal, to_pascal(n)),
        Kind::Scalar => format!("{n}: &str"),
        Kind::Slot => format!("{n}: Fragment"),
        Kind::Repeat => {
            let r = sig.repeats.iter().find(|r| r.coll == *n).expect("repeat");
            format!("{n}: &[{}{}]", fn_pascal, to_pascal(&r.var))
        }
    }).collect();

    out.push_str(&format!("pub fn {fn_name}({}) -> Fragment {{\n", params.join(", ")));
    out.push_str(&format!(
        "    let __html_src = ::std::fs::read_to_string({}).unwrap_or_else(|e| panic!(\"{fn_name}: failed to read template: {{e}}\"));\n",
        rust_str_lit(html_path),
    ));
    out.push_str(&format!(
        "    let mut __css = ::std::fs::read_to_string({}).unwrap_or_default();\n",
        rust_str_lit(css_path),
    ));
    out.push_str(&format!(
        "    let mut __js = ::std::fs::read_to_string({}).unwrap_or_default();\n",
        rust_str_lit(js_path),
    ));
    out.push_str("    let __html_parts = template::parse_template(&__html_src);\n");
    out.push_str("    let mut __vars: ::std::collections::HashMap<String, template::Value> = ::std::collections::HashMap::new();\n");

    for (name, kind) in &sig.root_order {
        match kind {
            Kind::Scalar => {
                out.push_str(&format!(
                    "    __vars.insert({}.to_string(), template::Value::Scalar({name}.to_string()));\n",
                    rust_str_lit(name),
                ));
            }
            Kind::Group => {
                let fields = &sig.groups.iter().find(|(n, _)| n == name).expect("group").1;
                out.push_str("    {\n");
                out.push_str("        let mut __g: ::std::collections::HashMap<String, String> = ::std::collections::HashMap::new();\n");
                for f in fields {
                    out.push_str(&format!(
                        "        __g.insert({}.to_string(), {name}.{f}.clone());\n",
                        rust_str_lit(f),
                    ));
                }
                out.push_str(&format!(
                    "        __vars.insert({}.to_string(), template::Value::Group(__g));\n",
                    rust_str_lit(name),
                ));
                out.push_str("    }\n");
            }
            Kind::Repeat => {
                let r = sig.repeats.iter().find(|r| r.coll == *name).expect("repeat");
                out.push_str("    {\n");
                out.push_str("        let mut __r: Vec<::std::collections::HashMap<String, String>> = Vec::new();\n");
                out.push_str(&format!("        for __row in {name}.iter() {{\n"));
                out.push_str("            let mut __g: ::std::collections::HashMap<String, String> = ::std::collections::HashMap::new();\n");
                for f in &r.fields {
                    out.push_str(&format!(
                        "            __g.insert({}.to_string(), __row.{f}.clone());\n",
                        rust_str_lit(f),
                    ));
                }
                out.push_str("            __r.push(__g);\n");
                out.push_str("        }\n");
                out.push_str(&format!(
                    "        __vars.insert({}.to_string(), template::Value::Repeat(__r));\n",
                    rust_str_lit(name),
                ));
                out.push_str("    }\n");
            }
            Kind::Slot => {
                out.push_str("    {\n");
                out.push_str(&format!("        if !{name}.css.is_empty() {{ if !__css.is_empty() {{ __css.push('\\n'); }} __css.push_str(&{name}.css); }}\n"));
                out.push_str(&format!("        if !{name}.js.is_empty() {{ if !__js.is_empty() {{ __js.push('\\n'); }} __js.push_str(&{name}.js); }}\n"));
                out.push_str(&format!(
                    "        __vars.insert({}.to_string(), template::Value::Slot({name}.html));\n",
                    rust_str_lit(name),
                ));
                out.push_str("    }\n");
            }
        }
    }

    out.push_str("    let mut __html = String::new();\n");
    if let Some(ld) = ld_path {
        out.push_str(&format!(
            "    {{\n        let __ld_src = ::std::fs::read_to_string({}).unwrap_or_default();\n        if !__ld_src.is_empty() {{\n            let __ld_parts = template::parse_template(&__ld_src);\n            __html.push_str(\"<script type=\\\"application/ld+json\\\">\");\n            __html.push_str(&template::render(&__ld_parts, &mut __vars, template::Escape::Json));\n            __html.push_str(\"</script>\");\n        }}\n    }}\n",
            rust_str_lit(ld),
        ));
    }
    out.push_str("    __html.push_str(&template::render(&__html_parts, &mut __vars, template::Escape::Html));\n");
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
