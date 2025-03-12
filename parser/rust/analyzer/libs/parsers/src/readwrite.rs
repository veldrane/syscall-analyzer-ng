use helpers::helpers::split_fd_parts;
use registry::registry::Parsable;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize,Deserialize)]
pub struct ReadWriteArgs {
    fd: i32,
    file_name: String,
    buffer: String,
    requested_size: i32,
    offset: String,
}

#[derive(Debug, Serialize,Deserialize)]
pub struct ReadWriteResults {
    size: i32,
}

#[typetag::serde]
impl Parsable for ReadWriteResults {
    fn parse(input: &str) -> Result<Self, String> {
        let size = input.parse::<i32>().map_err(|e| e.to_string())?;
        Ok(ReadWriteResults {
            size: size,
        })
    }   
}

#[typetag::serde]
impl Parsable for ReadWriteArgs {
    fn parse(input: &str) -> Result<Self, String> {

        let parts: Vec<String> = input
                                    .chars()
                                    .filter(|&c| !r#""\"? "#.contains(c))
                                    .collect::<String>()
                                    .split(',')
                                    .map(str::to_string)
                                    .collect::<Vec<String>>();

        if parts.len() < 3 {
            return Err("Invalid number of arguments".into());
        }
        
        let (fd, file_name ) = split_fd_parts(&parts[0]);

        let offset = if let Some(opt) = parts.get(3) {
            opt.to_string()
        } else {
            "".to_string()
        };

        Ok(ReadWriteArgs {
            fd: fd,
            file_name: file_name,
            buffer: parts[1].to_string(),
            requested_size:parts[2].parse::<i32>().unwrap(),
            offset: offset
        })
    }   
}