pub fn split_fd_parts(parts: &str) -> (i32, String) {
    let fd_parts: Vec<String> = parts
        .chars()
        .filter(|&c| !r#""\">? "#.contains(c))
        .collect::<String>()
        .split('<')
        .map(str::to_string)
        .collect::<Vec<String>>();
    let fd = fd_parts[0].parse::<i32>().unwrap();
    let filename = decode_hex_string(&fd_parts[1]);
    (fd, filename)
}


pub fn decode_hex_string(input: &str) -> String {
    let bytes: Vec<u8> = input
        .split("\\x") // Rozdělíme podle `\x`
        .filter(|s| !s.is_empty()) // Odstraníme prázdné části
        .map(|hex| u8::from_str_radix(hex, 16).unwrap()) // Parsujeme hex na u8
        .collect();

    String::from_utf8_lossy(&bytes).to_string() // Převedeme na String
}
