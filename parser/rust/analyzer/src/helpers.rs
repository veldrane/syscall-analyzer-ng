use std::str::FromStr;
use std::ops::Deref;
struct HexString(String);


pub fn split_fd_parts(parts: &str) -> (i32, String) {

    let fd_parts: Vec<String> = parts
    .chars()
    .filter(|&c| !r#""\">? "#.contains(c))
    .collect::<String>()
    .split('<')
    .map(str::to_string)
    .collect::<Vec<String>>();

    let fd = fd_parts[0].parse::<i32>().unwrap();
    let filename = HexString::from_str(&fd_parts[1]).unwrap().to_string();
    //let filename = HexString::fd_parts[1].clone(from_str().unwrap();
    (fd, filename)
}


impl FromStr for HexString {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes: Vec<u8> = s
            .split("x")
            .filter(|s| !s.is_empty())
            .map(|hex| u8::from_str_radix(hex, 16))
            .collect::<Result<_, _>>()?;
        Ok(HexString(String::from_utf8_lossy(&bytes).to_string()))
    }

}



impl Deref for HexString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
