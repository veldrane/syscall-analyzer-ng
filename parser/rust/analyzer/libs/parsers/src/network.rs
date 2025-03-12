//use crate::{helpers::split_fd_parts, registry::Parsable};

use helpers::helpers::split_fd_parts;
use registry::registry::Parsable;
use regex::Regex;
use serde::{Deserialize, Serialize};



//const ACCEPT_SYSCALL_ARGS: &str = r"(?P<socket_raw>\w+)\,\s\{(?P<socket_addr>\w+)\}\,\s(?P<socket_len>.*)\)";
const ACCEPT_SYSCALL_ARGS: &str = r"(?P<socket_raw>.*)\,\s*\{(?P<socket_addr>.*)\}\,\s(?P<socket_len>.*)";
//const ACCEPT_SYSCALL_ARGS: &str = r"(?P<socket_raw>\d+<socket:\[\d+\]>)\s*,\s*\{(?P<socket_addr>.*)\},\s*(?P<socket_len>.*)";
//const ACCEPT_SYSCALL_ARGS: &str = r"(?P<socket_raw>\d+<socket:\[\d+\]>),\s*\{(?P<socket_addr>[^}]+)\},\s*(?P<socket_len>[^,]+)";


#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkArgs {
    socket_fd: String,
    socket_name: String,
    socket_addr: String,
    socket_len: String,   
}

#[typetag::serde]
impl Parsable for NetworkArgs {
    fn parse(input: &str) -> Result<Self, String> {
        

        // let mut flags= 0;
        let re = Regex::new(ACCEPT_SYSCALL_ARGS).unwrap();
        let caps = re.captures(&input).unwrap();
        let (socket_fd, socket_name) = split_fd_parts(&caps["socket_raw"]);

        //if parts.len() != 4 {
        //    return Err("Invalid number of arguments".into());
        //}
        Ok(NetworkArgs {
            socket_fd: socket_fd.to_string(),
            socket_name: socket_name.to_string(),
            socket_addr: caps["socket_addr"].to_string(),
            socket_len: caps["socket_len"].to_string(),
        })
    }   
}