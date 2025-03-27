use serde::{Deserialize, Serialize};
use wrappers::parsers::Parsable;
use std::str::FromStr;
use helpers::helpers::{split_fd_parts_to_strings, split_fd_parts, HexString};


#[derive(Debug,Serialize,Deserialize, Default)]
pub struct OpenatArguments {
    dirfd: String,
    path: String,
    requested_file_name: String,
    flags: String,
    mode: String,
    fd: i32,
    file_name: String,
}


#[typetag::serde]
impl Parsable for OpenatArguments {
    fn parse(args: &str, result: Option<&str>) -> Result<Self, String> {

        let mut openat_syscall = OpenatArguments::default();

        let parts: Vec<String> = args
                                    .chars()
                                    .filter(|&c| !r#""\"? "#.contains(c))
                                    .collect::<String>()
                                    .split(',')
                                    .map(str::to_string)
                                    .collect::<Vec<String>>();
        if parts.len() < 3 {
            return Err("Invalid number of arguments".into());
        }

        let (dirfd, object) = split_fd_parts_to_strings(&parts[0]);

        if parts.len() == 4 {
            openat_syscall.mode = parts[3].to_string();
        }

        openat_syscall.dirfd = dirfd;
        openat_syscall.path = object;
        openat_syscall.requested_file_name = HexString::from_str(&parts[1]).unwrap().to_string();
        openat_syscall.flags = parts[2].to_string();

        (openat_syscall.fd, openat_syscall.file_name) = match result {
            Some(r) => split_fd_parts(&r),
            None => (0, "".to_string())
        };

        Ok(openat_syscall)
    }
}
