use std::fs;
use std::path::Path;

pub fn clean() {
    let dirs = ["dist", "pkg", "docs", "target"];
    for dir in &dirs {
        let path = Path::new(dir);
        if path.exists() {
            fs::remove_dir_all(path).unwrap_or_else(|e| {
                eprintln!("Warning: failed to remove {}: {}", dir, e);
            });
            println!("Removed: {}", dir);
        }
    }
}
