use serde::{Deserialize, Serialize};
use helpers::helpers::split_fd_parts;
use registry::registry::Parsable;

#[derive(Debug, Serialize,Deserialize)]
pub struct MmapArgs {
    addr: String,
    size: i32,
    protection: String,
    flags: String,
    fd: i32,
    file_name: String,
    offset: String
}


#[typetag::serde]
impl Parsable for MmapArgs {
    fn parse(input: &str) -> Result<Self, String> {

        let mut file_name = "".to_string();
        let mut fd = -1;

        let parts: Vec<String> = input
                                    .chars()
                                    .filter(|&c| !r#""\"? "#.contains(c))
                                    .collect::<String>()
                                    .split(',')
                                    .map(str::to_string)
                                    .collect::<Vec<String>>();

        if parts.len() != 6 {
            return Err("Invalid number of arguments".into());
        }

        if !parts[4].contains("-1") {
            (fd, file_name) = split_fd_parts(&parts[4]);
        }

        Ok(MmapArgs {
            addr: parts[0].to_string(),
            size: parts[1].parse::<i32>().unwrap_or(0),
            protection: parts[2].to_string(),
            flags: parts[3].to_string(),
            fd: fd,
            file_name: file_name,
            offset: parts[5].to_string()
        })
    }
}