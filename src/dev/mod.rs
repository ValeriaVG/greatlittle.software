use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant, SystemTime};

use macros::html_template;

use crate::html::template;
use crate::{blog, home, html::finalize};

pub const DEV_OUT: &str = ".dev-dist";

html_template!(reload_script, "src/dev/reload");

pub fn run(port: u16) -> std::io::Result<()> {
    let out_root = PathBuf::from(DEV_OUT);
    if out_root.exists() {
        fs::remove_dir_all(&out_root)?;
    }
    let version = Arc::new(AtomicU64::new(0));
    rebuild(&out_root, &version)?;

    let watch_paths = vec![
        PathBuf::from("content"),
        PathBuf::from("assets"),
        PathBuf::from("src/blog"),
        PathBuf::from("src/home"),
    ];
    {
        let v = Arc::clone(&version);
        let out = out_root.clone();
        thread::spawn(move || watcher_loop(watch_paths, out, v));
    }

    let listener = TcpListener::bind(("127.0.0.1", port))?;
    println!("dev server: http://127.0.0.1:{port} (serving {DEV_OUT})");
    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                let out = out_root.clone();
                let v = Arc::clone(&version);
                thread::spawn(move || {
                    let _ = serve(s, &out, &v);
                });
            }
            Err(e) => eprintln!("accept error: {e}"),
        }
    }
    Ok(())
}

fn rebuild(out_root: &Path, version: &AtomicU64) -> std::io::Result<()> {
    let blog_out = out_root.join("blog");
    if blog_out.exists() {
        fs::remove_dir_all(&blog_out)?;
    }
    fs::create_dir_all(out_root)?;
    let page = finalize(home::render());
    fs::write(out_root.join("index.html"), &page)?;
    blog::build(Path::new("content"), out_root, true)?;
    let assets = Path::new("assets");
    if assets.exists() {
        let dst = out_root.join("assets");
        if dst.exists() {
            fs::remove_dir_all(&dst)?;
        }
        copy_dir(assets, &dst)?;
    }
    version.fetch_add(1, Ordering::SeqCst);
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

fn watcher_loop(paths: Vec<PathBuf>, out: PathBuf, version: Arc<AtomicU64>) {
    let mut prev = mtimes(&paths);
    loop {
        thread::sleep(Duration::from_millis(250));
        let next = mtimes(&paths);
        if next == prev {
            continue;
        }
        prev = next;
        let t = Instant::now();
        match rebuild(&out, &version) {
            Ok(_) => println!("rebuilt in {:?}", t.elapsed()),
            Err(e) => eprintln!("rebuild error: {e}"),
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
        if let Ok(mt) = md.modified() {
            m.insert(p.to_path_buf(), mt);
        }
        return;
    }
    if md.is_dir() {
        let Ok(entries) = fs::read_dir(p) else { return };
        for e in entries.flatten() {
            walk_mtimes(&e.path(), m);
        }
    }
}

fn serve(mut stream: TcpStream, out: &Path, version: &AtomicU64) -> std::io::Result<()> {
    let mut buf = [0u8; 8192];
    let n = stream.read(&mut buf)?;
    let head = std::str::from_utf8(&buf[..n]).unwrap_or("");
    let Some(target) = head.split_whitespace().nth(1) else {
        return write_status(&mut stream, 400, "bad request");
    };
    let path = target.split('?').next().unwrap_or("/");

    if path == "/_reload" {
        let v = version.load(Ordering::SeqCst).to_string();
        return write_response(&mut stream, 200, "text/plain", v.as_bytes());
    }

    if path.contains("..") {
        return write_status(&mut stream, 403, "forbidden");
    }

    let trimmed = path.trim_start_matches('/');
    let mut file_path = if trimmed.is_empty() {
        out.to_path_buf()
    } else {
        out.join(trimmed)
    };
    if file_path.is_dir() {
        file_path = file_path.join("index.html");
    }

    let bytes = match fs::read(&file_path) {
        Ok(b) => b,
        Err(_) => return write_status(&mut stream, 404, "not found"),
    };
    let ct = content_type(&file_path);
    let body = if ct == "text/html" {
        inject_reload(&String::from_utf8_lossy(&bytes)).into_bytes()
    } else {
        bytes
    };
    write_response(&mut stream, 200, ct, &body)
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

fn write_response(stream: &mut TcpStream, code: u16, ct: &str, body: &[u8]) -> std::io::Result<()> {
    let reason = match code {
        200 => "OK",
        _ => "",
    };
    let header = format!(
        "HTTP/1.1 {code} {reason}\r\nContent-Type: {ct}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream.write_all(header.as_bytes())?;
    stream.write_all(body)?;
    Ok(())
}

fn write_status(stream: &mut TcpStream, code: u16, msg: &str) -> std::io::Result<()> {
    write_response(stream, code, "text/plain", msg.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_type_maps_known_extensions() {
        assert_eq!(content_type(Path::new("a.html")), "text/html");
        assert_eq!(content_type(Path::new("a.css")), "text/css");
        assert_eq!(content_type(Path::new("a.png")), "image/png");
        assert_eq!(content_type(Path::new("a.unknown")), "application/octet-stream");
    }

    #[test]
    fn inject_reload_places_script_before_body_close() {
        let out = inject_reload("<html><body>hi</body></html>");
        let script_idx = out.find("<script>").expect("script injected");
        assert!(script_idx < out.find("</body>").unwrap());
        assert!(out.contains(&reload_script_js()));
    }

    #[test]
    fn inject_reload_appends_when_no_body_close() {
        let out = inject_reload("plain");
        assert!(out.starts_with("plain<script>"));
        assert!(out.ends_with("</script>"));
    }
}

fn content_type(p: &Path) -> &'static str {
    match p.extension().and_then(|e| e.to_str()) {
        Some("html") => "text/html",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("json") => "application/json",
        Some("xml") => "application/xml",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        Some("ico") => "image/x-icon",
        Some("wav") => "audio/wav",
        Some("mp3") => "audio/mpeg",
        Some("ogg") => "audio/ogg",
        Some("woff") => "font/woff",
        Some("woff2") => "font/woff2",
        Some("ttf") => "font/ttf",
        Some("txt") | Some("md") => "text/plain",
        _ => "application/octet-stream",
    }
}
