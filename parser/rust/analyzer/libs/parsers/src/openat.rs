use serde::{Deserialize, Serialize};
use registry::registry::Parsable;
use std::str::FromStr;
use helpers::helpers::{split_fd_parts_to_strings, HexString,split_fd_parts};


#[derive(Debug,Serialize,Deserialize)]
pub struct OpenatArguments {
    dirfd: String,
    path: String,
    request: String,
    flags: String,
    mode: String,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct OpenatResults {
    pub fd: i32,
    pub file_name: String,
}

#[typetag::serde]
impl Parsable for OpenatArguments {
    fn parse(input: &str) -> Result<Self, String> {

        let mut openat_syscall = OpenatArguments {
            dirfd: "".to_string(),
            path: "".to_string(),
            request: "".to_string(),
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
        openat_syscall.request = HexString::from_str(&parts[1]).unwrap().to_string();
        openat_syscall.flags = parts[2].to_string();

        Ok(openat_syscall)
    }
}

#[typetag::serde]
impl Parsable for OpenatResults {
    fn parse(input: &str) -> Result<Self, String> {
        
        let parts: Vec<&str> = input
                                    .split(' ')
                                    .collect();


        if parts[0] == "-1" {
            return Err("Error opening file".into());
        }

        let (fd, file_name) = split_fd_parts(&parts[0]);

        Ok(OpenatResults {
            fd: fd,
            file_name: file_name,
        })
    }
}