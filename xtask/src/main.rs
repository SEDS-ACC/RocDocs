use mdbook_driver::MDBook;
use std::{env, fs, path::{Path, PathBuf}, os::unix::fs::symlink};

const INDEX_HTML: &str = include_str!("../../index.html");

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf();
    let dist = root.join("dist");

    build_workspace(&root, &dist);

    if args.contains(&"serve".to_string()) {
        println!("Serving RocDocs at http://localhost:8000");
        warp::serve(warp::fs::dir(dist)).run(([127, 0, 0, 1], 8000)).await;
    }
}

fn build_workspace(root: &Path, dist: &Path) {
    let books = vec![
        ("crates/L1", "L1"),
        ("crates/L2", "L2"),
        ("crates/L3", "L3"),
        ("crates/Team", "Team")
    ];

    if dist.exists() { fs::remove_dir_all(dist).ok(); }
    fs::create_dir_all(dist).ok();

    for (crate_path, dest_name) in &books {
        println!("Building {}...", dest_name);
        let crate_root = root.join(crate_path);
        
        // 1. Create temporary symlinks to the root assets
        let tmp_static = crate_root.join("static");
        let tmp_theme = crate_root.join("theme");

        // Clean up any stale links first
        let _ = fs::remove_file(&tmp_static);
        let _ = fs::remove_file(&tmp_theme);

        symlink(root.join("static"), &tmp_static).ok();
        symlink(root.join("theme"), &tmp_theme).ok();

        // 2. Build the book
        let mut md = MDBook::load(&crate_root).expect("Failed to load book");
        md.config.build.build_dir = dist.join(dest_name);
        md.build().expect("Build failed");

        // 3. Remove symlinks immediately - your source tree is clean again!
        let _ = fs::remove_file(&tmp_static);
        let _ = fs::remove_file(&tmp_theme);
    }

    fs::write(dist.join("index.html"), INDEX_HTML).ok();
    println!("Workspace build complete.");
}
