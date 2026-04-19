use std::fs;
use std::path::Path;

use greatlittle_software::{home, html::finalize};

fn main() -> std::io::Result<()> {
    let page = finalize(home::render());
    let dist = Path::new("dist");
    fs::create_dir_all(dist)?;
    let out = dist.join("index.html");
    fs::write(&out, &page)?;
    println!("wrote {}", out.display());
    Ok(())
}
