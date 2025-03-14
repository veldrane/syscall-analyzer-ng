use helpers::helpers::split_fd_parts;
use registry::registry::Parsable;
use regex::Regex;
use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;


//const ACCEPT_SYSCALL_ARGS: &str = r"(?P<socket_raw>\w+)\,\s\{(?P<socket_addr>\w+)\}\,\s(?P<socket_len>.*)\)";
const ACCEPT_SYSCALL_ARGS: &str = r"(?P<socket_raw>.*)\,\s*\{(?P<socket_addr>.*)\}\,\s(?P<socket_len>.*)";
//const ACCEPT_SYSCALL_ARGS: &str = r"(?P<socket_raw>\d+<socket:\[\d+\]>)\s*,\s*\{(?P<socket_addr>.*)\},\s*(?P<socket_len>.*)";
//const ACCEPT_SYSCALL_ARGS: &str = r"(?P<socket_raw>\d+<socket:\[\d+\]>),\s*\{(?P<socket_addr>[^}]+)\},\s*(?P<socket_len>[^,]+)";


static re: Lazy<Regex> = Lazy::new(|| Regex::new(ACCEPT_SYSCALL_ARGS).unwrap());

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkArgs {
    socket_fd: String,
    socket_name: String,
    socket_addr: String,
    socket_len: String,   
}

// socket addr must be described in more depth, not much knowledge about this parameter
#[derive(Debug, Serialize, Deserialize)]
pub struct Accept4Args {
    parrent_socket_fd: String,
    parrent_socket_name: String,
    socket_addr: String,
    socket_len: String,   
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Accept4Results {
    socket_fd: i32,
    socket_name: String,
}   

#[typetag::serde]
impl Parsable for NetworkArgs {
    fn parse(input: &str) -> Result<Self, String> {
        

        // let mut flags= 0;

        let caps = re.captures(&input).ok_or("Error network parsing")?;
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


#[typetag::serde]
impl Parsable for Accept4Args {
    fn parse(input: &str) -> Result<Self, String> {
        

        // let mut flags= 0;

        let caps = re.captures(&input).unwrap();
        let (socket_fd, socket_name) = split_fd_parts(&caps["socket_raw"]);

        //if parts.len() != 4 {
        //    return Err("Invalid number of arguments".into());
        //}
        Ok(Accept4Args {
            parrent_socket_fd: socket_fd.to_string(),
            parrent_socket_name: socket_name.to_string(),
            socket_addr: caps["socket_addr"].to_string(),
            socket_len: caps["socket_len"].to_string(),
        })
    }   
}


#[typetag::serde]
impl Parsable for Accept4Results {
    fn parse(input: &str) -> Result<Self, String> {
        
        let parts: Vec<&str> = input
                                    .split(' ')
                                    .collect();


        if parts[0] == "-1" {
            return Err("Error opening socket".into());
        }

        let (socket_fd, socket_name) = split_fd_parts(&parts[0]);

        Ok(Accept4Results {
            socket_fd: socket_fd,
            socket_name: socket_name,
        })
    }
}