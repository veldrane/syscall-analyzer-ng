use crate::{helpers::split_fd_parts, registry::SyscallArguments};
use regex::Regex;



//const ACCEPT_SYSCALL_ARGS: &str = r"(?P<socket_raw>\w+)\,\s\{(?P<sock_addr>\w+)\}\,\s(?P<sock_len>.*)\)";
const ACCEPT_SYSCALL_ARGS: &str = r"(?P<socket_raw>\d+<socket:\[\d+\]>)\s*,\s*\{(?P<sock_addr>.*)\},\s*(?P<sock_len>.*)";

#[derive(Debug)]
pub struct AcceptArgs {
    sockfd: String,
    sock_name: String,
    sock_addr: String,
    sock_len: String,   
}

impl SyscallArguments for AcceptArgs {
    fn parse(input: &str) -> Result<Self, String> {
        

        let mut flags= 0;

        let re = Regex::new(ACCEPT_SYSCALL_ARGS).unwrap();
        println!("input: {:?}", input);
        let caps = re.captures(input).unwrap();

        let (sockfd, sock_name) = split_fd_parts(&caps["socket_raw"]);

        //if parts.len() != 4 {
        //    return Err("Invalid number of arguments".into());
        //}
        Ok(AcceptArgs {
            sockfd: sockfd.to_string(),
            sock_name: sock_name,
            sock_addr: caps["sock_addr"].to_string(),
            sock_len: caps["sock_len"].to_string(),
        })
    }   
}