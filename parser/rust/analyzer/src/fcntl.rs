use crate::{helpers::split_fd_parts, registry::SyscallArguments};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize,Deserialize)]
pub struct FcntlArgs {
    fd: i32,
    file_name: String,
    operation: String,
    opt_arg: String,
}


#[typetag::serde]
impl SyscallArguments for FcntlArgs {
    fn parse(input: &str) -> Result<Self, String> {

        let parts: Vec<String> = input
                                    .chars()
                                    .filter(|&c| !r#""\"? "#.contains(c))
                                    .collect::<String>()
                                    .split(',')
                                    .map(str::to_string)
                                    .collect::<Vec<String>>();

        if parts.len() < 2 {
            return Err("Invalid number of arguments".into());
        }
        let (fd, file_name ) = split_fd_parts(&parts[0]);

        let opt_arg = if let Some(opt) = parts.get(2) {
            opt.to_string()
        } else {
            "".to_string()
        };

        Ok(FcntlArgs {
            fd: fd,
            file_name: file_name,
            operation: parts[1].to_string(),
            opt_arg: opt_arg
        })
    }   
}