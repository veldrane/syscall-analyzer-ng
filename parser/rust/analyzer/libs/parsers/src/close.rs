use helpers::helpers::split_fd_parts;
use registry::registry::Parsable;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize,Deserialize, Default)]
pub struct CloseArgs {
    fd: i32,
    file_name: String,
}


#[typetag::serde]
impl Parsable for CloseArgs {
    fn parse(input: &str) -> Result<Self, String> {


        match input {
            "-1" => return Err("Error closing file".into()),
            s if !s.contains("<") => return Err("Invalid file descriptor".into()),
            s if s.contains("deleted") => return Err("Invalid file descriptor".into()),
            _ => (),
        }

        let (fd, file_name ) = split_fd_parts(&input);

        Ok(CloseArgs {
            fd: fd,
            file_name: file_name,
        })
    }   
}