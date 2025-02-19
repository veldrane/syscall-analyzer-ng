fn decode_hex_string(input: &str) -> String {
    let bytes: Vec<u8> = input
        .split("\\x") // Rozdělíme podle `\x`
        .filter(|s| !s.is_empty()) // Odstraníme prázdné části
        .map(|hex| u8::from_str_radix(hex, 16).unwrap()) // Parsujeme hex na u8
        .collect();

    String::from_utf8_lossy(&bytes).to_string() // Převedeme na String
}

fn main() {
    let input = r"\x2f\x64\x65\x76\x2f\x70\x74\x73\x2f\x31";
    let output = decode_hex_string(input);
    
    println!("{}", output); // Výstup: "/dev/pts/1"
}
