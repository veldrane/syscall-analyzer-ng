use crate::{helpers::split_fd_parts, registry::SyscallArguments};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize,Deserialize)]
pub struct EpollCreateArgs {
    size: i32,
}


#[typetag::serde]
impl SyscallArguments for EpollCreateArgs {
    fn parse(input: &str) -> Result<Self, String> {

        Ok(EpollCreateArgs {
            size: input.parse::<i32>().unwrap(),
        })
    }   
}

#[derive(Debug, Serialize,Deserialize)]
pub struct EpollCreate1Args {
    flags: String,
}


#[typetag::serde]
impl SyscallArguments for EpollCreate1Args {
    fn parse(input: &str) -> Result<Self, String> {

        Ok(EpollCreate1Args {
            flags: input.to_string(),
        })
    }   
}