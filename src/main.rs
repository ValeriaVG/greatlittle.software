use std::fs;
use std::path::{Path, PathBuf};

use greatlittle_software::{blog, dev, home, html::finalize};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(|s| s.as_str()) == Some("dev") {
        let port = args
            .get(2)
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(8000);
        return dev::run(port);
    }

    let page = finalize(home::render());
    let dist = Path::new("dist");
    fs::create_dir_all(dist)?;
    let out = dist.join("index.html");
    fs::write(&out, &page)?;
    println!("wrote {}", out.display());

    let content = Path::new("content");
    for written in blog::build(content, dist, false)? {
        println!("wrote {written}");
    }

    let assets = Path::new("assets");
    if assets.exists() {
        copy_dir(assets, &dist.join("assets"))?;
        println!("copied {} to {}", assets.display(), dist.join("assets").display());
    }
    Ok(())
}

fn copy_dir(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let from = entry.path();
        let to: PathBuf = dst.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir(&from, &to)?;
        } else {
            fs::copy(&from, &to)?;
        }
    }
    Ok(())
}
