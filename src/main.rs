use std::fs;
use std::path::Path;
use std::io;
use std::collections::HashSet;

/// Recursively print directory structure
fn print_dir_structure<P: AsRef<Path>>(path: P, prefix: String) {
    if let Ok(entries) = fs::read_dir(&path) {
        let ignored_dirs: HashSet<_> = ["target", "node_modules", "dist", "build", "coverage"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let mut entries: Vec<_> = entries
            .filter_map(Result::ok)
            .filter(|entry| {
                let file_name = entry.file_name();
                let name = file_name.to_string_lossy();
                !name.starts_with('.') && !ignored_dirs.contains(&name.to_string())
            })
            .collect();
            
        entries.sort_by_key(|entry| {
            let is_dir = entry.path().is_dir();
            // Sort directories first, then files
            (!is_dir, entry.file_name())
        });

        for (i, entry) in entries.iter().enumerate() {
            let is_last = i == entries.len() - 1;
            let new_prefix = if is_last { "└── " } else { "├── " };
            
            // Get file name
            let file_name = entry.file_name();
            let name = file_name.to_string_lossy();
            println!("{}{}{}", prefix, new_prefix, name);

            if entry.path().is_dir() {
                let next_prefix = if is_last {
                    format!("{}    ", prefix)
                } else {
                    format!("{}│   ", prefix)
                };
                print_dir_structure(entry.path(), next_prefix);
            }
        }
    } else {
        eprintln!("Could not read directory: {}", path.as_ref().display());
    }
}

fn main() {
    println!("Please input your directory:");
    let mut path = String::new();
    
    io::stdin()
        .read_line(&mut path)
        .expect("Failed to read line");

    let path = path.trim();
    let path = if !path.starts_with('/') {
        format!("/{}", path)
    } else {
        path.to_string()
    };
    
    print_dir_structure(path, "".to_string());
}