use std::ops::Deref;

use crate::{helpers::split_fd_parts, registry::SyscallArguments};
use regex::Regex;
use serde::{Deserialize, Serialize};


//const ACCEPT_SYSCALL_ARGS: &str = r"(?P<socket_raw>\w+)\,\s\{(?P<sock_addr>\w+)\}\,\s(?P<sock_len>.*)\)";
const ACCEPT_SYSCALL_ARGS: &str = r"(?P<socket_raw>.*)\,\s*\{(?P<sock_addr>.*)\}\,\s(?P<sock_len>.*)";
//const ACCEPT_SYSCALL_ARGS: &str = r"(?P<socket_raw>\d+<socket:\[\d+\]>)\s*,\s*\{(?P<sock_addr>.*)\},\s*(?P<sock_len>.*)";
//const ACCEPT_SYSCALL_ARGS: &str = r"(?P<socket_raw>\d+<socket:\[\d+\]>),\s*\{(?P<sock_addr>[^}]+)\},\s*(?P<sock_len>[^,]+)";


#[derive(Debug, Serialize, Deserialize)]
pub struct AcceptArgs {
    sockfd: String,
    sock_name: String,
    sock_addr: String,
    sock_len: String,   
}

#[typetag::serde]
impl SyscallArguments for AcceptArgs {
    fn parse(input: &str) -> Result<Self, String> {
        

        // let mut flags= 0;
        let re = Regex::new(ACCEPT_SYSCALL_ARGS).unwrap();
        let caps = re.captures(&input).unwrap();
        let (sockfd, sock_name) = split_fd_parts(&caps["socket_raw"]);

        //if parts.len() != 4 {
        //    return Err("Invalid number of arguments".into());
        //}
        Ok(AcceptArgs {
            sockfd: sockfd.to_string(),
            sock_name: sock_name.to_string(),
            sock_addr: caps["sock_addr"].to_string(),
            sock_len: caps["sock_len"].to_string(),
        })
    }   
}