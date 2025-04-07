use std::fs::{File, DirEntry};
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;



pub fn find_first(dir_path: &str) -> Option<(String, f64)> {

    let path = Path::new(dir_path);

    let entries = path.read_dir()
    .expect("Failed to read directory")
    .filter_map(Result::ok)
    .filter(|entry| entry.path().is_file());

    for entry in entries {

        println!("Checking file: {:?}", entry.path());

        match find_first_ts(&entry) {
            Some(ts) => {
                return Some((entry.path().to_string_lossy().to_string(), ts));
            },
            None => {
                continue
            }
        };
        
    }
    None
}

fn find_first_ts(dir_entry: &DirEntry) -> Option<f64> {

    let f = File::open(dir_entry.path()).expect("Failed to open file");
    let file = BufReader::new(f);
    let line = file.lines().next();

    match &line {
        Some(Ok(line)) => {
            if line.contains("execve") {
                let result = line.split_whitespace()
                    .filter_map(|s| s.parse::<f64>().ok())
                    .next();

                return result;
            }
        },
        _ => {}
    }
    return None

}