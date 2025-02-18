pub fn split_fd_parts(parts: &str) -> (i32, String) {
    let fd_parts: Vec<String> = parts
        .chars()
        .filter(|&c| !r#""\">? "#.contains(c))
        .collect::<String>()
        .split('<')
        .map(str::to_string)
        .collect::<Vec<String>>();
    let fd = fd_parts[0].parse::<i32>().unwrap();
    let filename = fd_parts[1].to_string();
    (fd, filename)
}