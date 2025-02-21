use serde::{Deserialize, Serialize};
use crate::helpers::{split_fd_parts_to_strings, HexString};
use std::str::FromStr;

use crate::registry::SyscallArguments;

#[derive(Debug,Serialize,Deserialize)]
pub struct OpenatArguments {
    dirfd: String,
    path: String,
    filename: String,
    flags: String,
    mode: String,
}


#[typetag::serde]
impl SyscallArguments for OpenatArguments {
    fn parse(input: &str) -> Result<Self, String> {

        let mut openat_syscall = OpenatArguments {
            dirfd: "".to_string(),
            path: "".to_string(),
            filename: "".to_string(),
            flags: "".to_string(),
            mode: "".to_string(),
        };
        
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

        //println!("parts vystup{:?}", parts);

        let (dirfd, object) = split_fd_parts_to_strings(&parts[0]);

        if parts.len() == 4 {
            openat_syscall.mode = parts[3].to_string();
        }

        openat_syscall.dirfd = dirfd;
        openat_syscall.path = object;
        openat_syscall.filename = HexString::from_str(&parts[1]).unwrap().to_string();
        openat_syscall.flags = parts[2].to_string();

        Ok(openat_syscall)
    }
}