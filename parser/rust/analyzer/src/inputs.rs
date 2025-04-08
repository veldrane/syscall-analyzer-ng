use std::fs::{File, DirEntry};
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;



pub fn find_first(dir_path: &str) -> Option<(i32, f64)> {

    let path = Path::new(dir_path);

    let entries = path.read_dir()
    .expect("Failed to read directory")
    .filter_map(Result::ok)
    .filter(|entry| entry.path().is_file());

    for entry in entries {

        let path_buf = entry.path();
        let path_str = path_buf.to_str().unwrap_or("");
        let parts: Vec<&str> = path_str.split(".").collect();

        let pid = parts[parts.len() - 1]
            .split(".")
            .collect::<Vec<&str>>()
            .last()
            .unwrap_or(&"0")
            .parse::<i32>()
            .unwrap_or(0);

        match find_first_ts(&entry) {
            Some(ts) => {
                return Some((pid, ts));
            },
            None => {
                continue
            }
        };
        
    }
    None
}

fn find_first_ts(dir_entry: &DirEntry) -> Option<(f64)> {

    let f = File::open(dir_entry.path()).expect("Failed to open file");
    let file = BufReader::new(f);
    let line = file.lines().next();


    match &line {
        Some(Ok(line)) => {
            if line.contains("execve") {
                let ts = line.split_whitespace()
                    .filter_map(|s| s.parse::<f64>().ok())
                    .next();

                return ts;
            }
        },
        _ => {}
    }
    return None

}