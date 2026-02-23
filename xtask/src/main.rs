use mdbook_driver::MDBook;
use mdbook_html::HtmlHandlebars;
use std::env;
use std::fs;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    let xtask_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = xtask_dir.parent().unwrap().to_path_buf();
    let dist = root.join("dist");

    println!("Building Workspace (Native 0.5.2)...");

    if dist.exists() {
        fs::remove_dir_all(&dist).unwrap();
    }
    fs::create_dir_all(&dist).unwrap();

    let books = /* iterate over crates */
        vec![
        ("crates/L1", "v1"),
        ("crates/L2", "v2"),
        ("crates/L3", "v3"),
    ];

    for (src, dest) in books {
        let book_dir = root.join(src);
        if !book_dir.exists() { continue; }

        println!("Compiling {}...", src);
        let mut md = MDBook::load(&book_dir)
            .expect(&format!("Failed to load mdBook at {:?}", book_dir));

        md.config.build.build_dir = dist.join(dest);
        md.with_renderer(HtmlHandlebars::new());
        md.build().expect("Build failed");
    }

    fs::copy(root.join("index.html"), dist.join("index.html")).ok();
    let static_dir = root.join("static");
    let dist_static = dist.join("static");
    if static_dir.exists() {
        fs::create_dir_all(&dist_static).unwrap();
        for entry in fs::read_dir(static_dir).unwrap().filter_map(|e| e.ok()) {
            if entry.path().is_file() {
                fs::copy(entry.path(), dist_static.join(entry.file_name())).ok();
            }
        }
    }
    println!("Workspace built successfully.");

    // ---- The Native Rust Server ----
    let args: Vec<String> = env::args().collect();
    if args.contains(&"serve".to_string()) {
        println!("Serving RocDocs natively at http://localhost:8000");
        println!("Press Ctrl+C to stop.");
        
        let route = warp::fs::dir(dist);
        warp::serve(route).run(([127, 0, 0, 1], 8000)).await;
    }
}
