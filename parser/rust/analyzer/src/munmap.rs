use serde::{Deserialize, Serialize};

use crate::registry::SyscallArguments;
use crate::helpers::split_fd_parts;

#[derive(Debug, Serialize,Deserialize)]
pub struct MunmapArguments {
    addr: String,
    size: i32,
}


#[typetag::serde]
impl SyscallArguments for MunmapArguments {
    fn parse(input: &str) -> Result<Self, String> {


        let parts: Vec<String> = input
                                    .chars()
                                    .filter(|&c| !r#""\"? "#.contains(c))
                                    .collect::<String>()
                                    .split(',')
                                    .map(str::to_string)
                                    .collect::<Vec<String>>();

        Ok(MunmapArguments {
            addr: parts[0].to_string(),
            size: parts[1].parse::<i32>().unwrap(),
        })
    }
}