use registry::registry::SyscallArguments;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize,Deserialize)]
pub struct EpollCreateArgs {
    size: i32,
}


#[typetag::serde]
impl SyscallArguments for EpollCreateArgs {
    fn parse(input: &str) -> Result<Self, String> {

        let size = input.parse::<i32>().map_err(|e| e.to_string())?;

        Ok(EpollCreateArgs {
            size: size,
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