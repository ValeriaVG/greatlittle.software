fn main() {
    let content = std::path::Path::new("content");
    let out = std::path::Path::new("/tmp/test-blog-out");
    match greatlittle_software::blog::build(content, out, true) {
        Ok(files) => { for f in files { println!("{f}"); } }
        Err(e) => eprintln!("ERR: {e}"),
    }
}
