use helpers::helpers::split_fd_parts;
use registry::registry::SyscallArguments;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize,Deserialize)]
pub struct CloseArgs {
    fd: i32,
    file_name: String,
}


#[typetag::serde]
impl SyscallArguments for CloseArgs {
    fn parse(input: &str) -> Result<Self, String> {

        let (fd, file_name ) = split_fd_parts(&input);

        Ok(CloseArgs {
            fd: fd,
            file_name: file_name,
        })
    }   
}