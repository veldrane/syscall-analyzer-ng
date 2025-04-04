use std::fs::{File, DirEntry};
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;



pub fn find_first(dir_path: &str) -> Option<String> {

    let path = Path::new(dir_path);

    let entries = path.read_dir()
    .expect("Failed to read directory")
    .filter_map(Result::ok)
    .filter(|entry| entry.path().is_file());

    for entry in entries {

        if is_first(&entry) {
            return Some(entry.path().to_string_lossy().to_string());
        } 
        
    }
    None
}

fn is_first(dir_entry: &DirEntry) -> bool {

    let f = File::open(dir_entry.path()).expect("Failed to open file");
    let file = BufReader::new(f);
    let line = file.lines().next();

    match line {
        Some(Ok(line)) => {
            if line.contains("execve") {
                return true
            }
        },
        _ => {}
    }

    return false

}

