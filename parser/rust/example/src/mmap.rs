use crate::registry::SyscallArguments;

#[derive(Debug)]
pub struct MmapArguments {
    addr: String,
    size: i32,
    protection: String,
    flags: String,
    fd: i32,
    filename: String,
    offset: String
}


impl SyscallArguments for MmapArguments {
    fn parse(input: &str) -> Result<Self, String> {

        let mut filename = "".to_string();
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
            let fd_parts: Vec<String> = parts[4]
            .chars()
            .filter(|&c| !r#""\">? "#.contains(c))
            .collect::<String>()
            .split('<')
            .map(str::to_string)
            .collect::<Vec<String>>();
            fd = fd_parts[0].parse::<i32>().unwrap();
            filename = fd_parts[1].to_string();
        }

        Ok(MmapArguments {
            addr: parts[0].to_string(),
            size: parts[1].parse::<i32>().unwrap(),
            protection: parts[2].to_string(),
            flags: parts[3].to_string(),
            fd: fd,
            filename: filename,
            offset: parts[5].to_string()
        })
    }
}