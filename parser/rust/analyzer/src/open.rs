use serde::{Deserialize, Serialize};
use crate::helpers::split_fd_parts;

use crate::registry::SyscallArguments;

#[derive(Debug,Serialize,Deserialize)]
pub struct OpenArguments {
    dirfd: String,
    object: String,
    mode: String,
}


#[typetag::serde]
impl SyscallArguments for OpenArguments {
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
        if parts.len() < 3 {
            return Err("Invalid number of arguments".into());
        }

        //if !parts[0].contains("-1") {
        //    (fd, filename) = split_fd_parts(&parts[0]);
        //}

        Ok(OpenArguments {
            dirfd: parts[0].to_string(),
            object: parts[1].to_string(),
            mode: parts[2].to_string()
        })
    }
}