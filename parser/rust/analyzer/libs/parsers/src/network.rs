use helpers::helpers::split_fd_parts;
use wrappers::parsers::Parsable;
use regex::Regex;
use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use std::any::Any;
use std::rc::Rc;

//const ACCEPT_SYSCALL_ARGS: &str = r"(?P<socket_raw>\w+)\,\s\{(?P<socket_addr>\w+)\}\,\s(?P<socket_len>.*)\)";
const ACCEPT_SYSCALL_ARGS: &str = r"(?P<socket_raw>.*)\,\s*\{(?P<socket_addr>.*)\}\,\s(?P<socket_len>.*)";
//const ACCEPT_SYSCALL_ARGS: &str = r"(?P<socket_raw>\d+<socket:\[\d+\]>)\s*,\s*\{(?P<socket_addr>.*)\},\s*(?P<socket_len>.*)";
//const ACCEPT_SYSCALL_ARGS: &str = r"(?P<socket_raw>\d+<socket:\[\d+\]>),\s*\{(?P<socket_addr>[^}]+)\},\s*(?P<socket_len>[^,]+)";


static RE: Lazy<Regex> = Lazy::new(|| Regex::new(ACCEPT_SYSCALL_ARGS).unwrap());

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkArgs {
    socket_fd: i32,
    socket_name: String,
    socket_addr: String,
    socket_len: i32,   
}

// socket addr must be described in more depth, not much knowledge about this parameter
#[derive(Debug, Serialize, Deserialize)]
pub struct Accept4Args {
    parrent_socket_fd: i32,
    parrent_socket_name: String,
    socket_addr: String,
    socket_len: i32,
    socket_fd: i32,
    socket_name: String,  
}

#[typetag::serde]
impl Parsable for NetworkArgs {
    fn parse(args: &str,_ : Option<&str>) -> Result<Self, String> {
        

        // let mut flags= 0;

        let caps = RE.captures(&args).ok_or("Error network parsing")?;
        let (socket_fd, socket_name) = split_fd_parts(&caps["socket_raw"]);

        //if parts.len() != 4 {
        //    return Err("Invalid number of arguments".into());
        //}
        Ok(NetworkArgs {
            socket_fd: socket_fd,
            socket_name: socket_name.to_string(),
            socket_addr: caps["socket_addr"].to_string(),
            socket_len: caps["socket_len"].parse::<i32>().unwrap_or(0),
        })
    }   

    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }

}


#[typetag::serde]
impl Parsable for Accept4Args {
    fn parse(args: &str, result: Option<&str>) -> Result<Self, String> {
        

        // let mut flags= 0;

        let caps = RE.captures(&args).unwrap();
        let (parrent_socket_fd, parrent_socket_name) = split_fd_parts(&caps["socket_raw"]);

        let (socket_fd, socket_name) = match result {
            Some(r) => split_fd_parts(&r),
            None => (0, "".to_string()) 
        };


        Ok(Accept4Args {
            parrent_socket_fd: parrent_socket_fd,
            parrent_socket_name: parrent_socket_name.to_string(),
            socket_addr: caps["socket_addr"].to_string(),
            socket_len: caps["socket_len"].parse::<i32>().unwrap_or(0),
            socket_fd: socket_fd,
            socket_name: socket_name,
        })
    }

    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}

