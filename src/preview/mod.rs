use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, SystemTime};

use macros::html_template;

use crate::dev::reload_script_js;
use crate::html::{Bundle, finalize, template};

pub trait IntoPreview {
    fn into_preview(self) -> Bundle;
}

impl IntoPreview for Bundle {
    fn into_preview(self) -> Bundle {
        self
    }
}

impl IntoPreview for String {
    fn into_preview(self) -> Bundle {
        Bundle { html: self, css: String::new(), js: String::new() }
    }
}

pub struct Preview {
    pub path: &'static str,
    pub render: fn() -> Bundle,
}

inventory::collect!(Preview);

html_template!(index_page, "src/preview/index");
html_template!(wrapper_page, "src/preview/wrapper");

pub const PREVIEW_OUT: &str = ".preview-dist";
const GLOBALS_CSS: &str = "src/preview/globals.css";
const GLOBALS_JS: &str = "src/preview/globals.js";
const ASSETS_DIR: &str = "public";

fn sorted_previews() -> Vec<&'static Preview> {
    let mut v: Vec<&Preview> = inventory::iter::<Preview>().collect();
    v.sort_by_key(|p| p.path);
    v
}

// Reads a file, expanding `/* @include PATH */` directives with the contents
// of PATH (resolved from the project root). One level deep; cycles left to the
// author to avoid.
fn load_with_includes(path: &str) -> String {
    let src = fs::read_to_string(path).unwrap_or_default();
    let mut out = String::with_capacity(src.len());
    for line in src.lines() {
        if let Some(rest) = line.trim().strip_prefix("/* @include ") {
            if let Some(target) = rest.strip_suffix("*/") {
                let target = target.trim();
                let included = fs::read_to_string(target).unwrap_or_default();
                out.push_str(&included);
                out.push('\n');
                continue;
            }
        }
        out.push_str(line);
        out.push('\n');
    }
    out
}

fn global_css() -> String { load_with_includes(GLOBALS_CSS) }
fn global_js() -> String { load_with_includes(GLOBALS_JS) }

fn prepend(target: &mut String, extra: &str) {
    if extra.is_empty() { return; }
    if target.is_empty() {
        target.push_str(extra);
    } else {
        let mut combined = String::with_capacity(extra.len() + 1 + target.len());
        combined.push_str(extra);
        combined.push('\n');
        combined.push_str(target);
        *target = combined;
    }
}

fn render_index() -> String {
    let rows = build_index_rows(sorted_previews().iter().map(|p| p.path));
    let html = index_page(&rows);
    let mut css = index_page_css();
    let mut js = index_page_js();
    prepend(&mut css, &global_css());
    prepend(&mut js, &global_js());
    finalize(Bundle { html, css, js })
}

fn build_index_rows<'a>(paths: impl IntoIterator<Item = &'a str>) -> Vec<IndexPageP> {
    let mut rows = Vec::new();
    let mut prev: Vec<&str> = Vec::new();
    for path in paths {
        let segs: Vec<&str> = path.split('/').collect();
        let (label_idx, group_segs) = match segs.len() {
            0 => continue,
            1 => (0, &segs[..0]),
            n => (n - 1, &segs[..n - 1]),
        };
        let common = prev.iter().zip(group_segs.iter()).take_while(|(a, b)| a == b).count();
        for (i, seg) in group_segs.iter().enumerate().skip(common) {
            rows.push(IndexPageP {
                group: seg.to_string(),
                href: String::new(),
                label: String::new(),
                depth: i.to_string(),
            });
        }
        rows.push(IndexPageP {
            group: String::new(),
            href: path.to_string(),
            label: segs[label_idx].to_string(),
            depth: group_segs.len().to_string(),
        });
        prev = group_segs.to_vec();
    }
    rows
}

fn render_preview(name: &str) -> Option<String> {
    let p = inventory::iter::<Preview>().find(|p| p.path == name)?;
    let body = (p.render)();
    let mut page = wrapper_page(p.path, body);
    prepend(&mut page.css, &global_css());
    prepend(&mut page.js, &global_js());
    Some(finalize(page))
}

fn inject_reload(html: &str) -> String {
    let tag = format!("<script>{}</script>", reload_script_js());
    if let Some(i) = html.rfind("</body>") {
        let mut out = String::with_capacity(html.len() + tag.len());
        out.push_str(&html[..i]);
        out.push_str(&tag);
        out.push_str(&html[i..]);
        out
    } else {
        format!("{html}{tag}")
    }
}

pub fn run(port: u16) -> std::io::Result<()> {
    let version = Arc::new(AtomicU64::new(0));
    let watch_paths = vec![PathBuf::from("src"), PathBuf::from("public")];
    {
        let v = Arc::clone(&version);
        thread::spawn(move || watcher_loop(watch_paths, v));
    }
    let listener = TcpListener::bind(("127.0.0.1", port))?;
    println!("preview server: http://127.0.0.1:{port}");
    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                let v = Arc::clone(&version);
                thread::spawn(move || { let _ = serve(s, &v); });
            }
            Err(e) => eprintln!("accept error: {e}"),
        }
    }
    Ok(())
}

fn watcher_loop(paths: Vec<PathBuf>, version: Arc<AtomicU64>) {
    let mut prev = mtimes(&paths);
    loop {
        thread::sleep(Duration::from_millis(250));
        let next = mtimes(&paths);
        if next != prev {
            prev = next;
            version.fetch_add(1, Ordering::SeqCst);
        }
    }
}

fn mtimes(paths: &[PathBuf]) -> HashMap<PathBuf, SystemTime> {
    let mut m = HashMap::new();
    for p in paths {
        walk_mtimes(p, &mut m);
    }
    m
}

fn walk_mtimes(p: &Path, m: &mut HashMap<PathBuf, SystemTime>) {
    let Ok(md) = fs::metadata(p) else { return };
    if md.is_file() {
        if let Ok(mt) = md.modified() { m.insert(p.to_path_buf(), mt); }
        return;
    }
    if md.is_dir() {
        let Ok(entries) = fs::read_dir(p) else { return };
        for e in entries.flatten() {
            walk_mtimes(&e.path(), m);
        }
    }
}

fn serve(mut stream: TcpStream, version: &AtomicU64) -> std::io::Result<()> {
    let mut buf = [0u8; 8192];
    let n = stream.read(&mut buf)?;
    let head = std::str::from_utf8(&buf[..n]).unwrap_or("");
    let Some(target) = head.split_whitespace().nth(1) else {
        return write_resp(&mut stream, 400, "text/plain", b"bad request");
    };
    let path = target.split('?').next().unwrap_or("/");
    if path.contains("..") {
        return write_resp(&mut stream, 403, "text/plain", b"forbidden");
    }
    if path == "/_reload" {
        let v = version.load(Ordering::SeqCst).to_string();
        return write_resp(&mut stream, 200, "text/plain", v.as_bytes());
    }
    if path == "/" || path.is_empty() {
        let body = inject_reload(&render_index());
        return write_resp(&mut stream, 200, "text/html", body.as_bytes());
    }
    if let Some(rest) = path.strip_prefix("/p/") {
        let name = url_decode(rest.trim_end_matches('/'));
        if let Some(body) = render_preview(&name) {
            return write_resp(&mut stream, 200, "text/html", inject_reload(&body).as_bytes());
        }
    }
    let relative = path.strip_prefix('/').unwrap_or(path);
    let asset = PathBuf::from(ASSETS_DIR).join(relative);
    if let Ok(bytes) = fs::read(&asset) {
        return write_resp(&mut stream, 200, content_type(&asset), &bytes);
    }
    write_resp(&mut stream, 404, "text/plain", b"not found")
}

fn url_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let (Some(h), Some(l)) = (hex(bytes[i + 1]), hex(bytes[i + 2])) {
                out.push((h << 4) | l);
                i += 3;
                continue;
            }
        }
        if bytes[i] == b'+' {
            out.push(b' ');
        } else {
            out.push(bytes[i]);
        }
        i += 1;
    }
    String::from_utf8(out).unwrap_or_else(|_| s.to_string())
}

fn hex(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

fn content_type(p: &Path) -> &'static str {
    match p.extension().and_then(|e| e.to_str()) {
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        Some("ico") => "image/x-icon",
        Some("woff") => "font/woff",
        Some("woff2") => "font/woff2",
        Some("ttf") => "font/ttf",
        _ => "application/octet-stream",
    }
}

fn write_resp(stream: &mut TcpStream, code: u16, ct: &str, body: &[u8]) -> std::io::Result<()> {
    let reason = match code {
        200 => "OK",
        400 => "Bad Request",
        403 => "Forbidden",
        404 => "Not Found",
        _ => "",
    };
    let header = format!(
        "HTTP/1.1 {code} {reason}\r\nContent-Type: {ct}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
        body.len(),
    );
    stream.write_all(header.as_bytes())?;
    stream.write_all(body)?;
    Ok(())
}

pub fn build(out: &Path) -> std::io::Result<()> {
    if out.exists() { fs::remove_dir_all(out)?; }
    fs::create_dir_all(out)?;
    fs::write(out.join("index.html"), render_index())?;
    let previews_root = out.join("p");
    for p in sorted_previews() {
        let page_dir = previews_root.join(p.path);
        fs::create_dir_all(&page_dir)?;
        let body = render_preview(p.path).expect("preview missing after lookup");
        fs::write(page_dir.join("index.html"), body)?;
    }
    let assets = Path::new(ASSETS_DIR);
    if assets.exists() {
        copy_dir(assets, &out.join("assets"))?;
    }
    Ok(())
}

fn copy_dir(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir(&from, &to)?;
        } else {
            fs::copy(&from, &to)?;
        }
    }
    Ok(())
}
