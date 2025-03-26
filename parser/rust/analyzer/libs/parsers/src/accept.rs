//use crate::{helpers::split_fd_parts, registry::Parsable};

use helpers::helpers::split_fd_parts;
// use registry::registry::Parsable;
use regex::Regex;
use serde::{Deserialize, Serialize};
use wrappers::parsers::Parsable;



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
impl Parsable for AcceptArgs {
    fn parse(args: &str, _: Option<&str>) -> Result<Self, String> {
        

        // let mut flags= 0;
        let re = Regex::new(ACCEPT_SYSCALL_ARGS).unwrap();
        let caps = re.captures(&args).unwrap();
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