use helpers::helpers::split_fd_parts;
use wrappers::parsers::Parsable;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::rc::Rc;


#[derive(Debug, Serialize,Deserialize)]
pub struct SocketTransferAttrs {
    socket_fd: i32,
    socket_name: String,
    buffer: String,
    size: i32,
    flags: String,
    socket_addr: String,
    socket_len: i32,
}


#[typetag::serde]
impl Parsable for SocketTransferAttrs {
    fn parse(args: &str, _: Option<&str>) -> Result<Self, String> {

        let parts: Vec<String> = args
                                    .chars()
                                    .filter(|&c| !r#""\"? "#.contains(c))
                                    .collect::<String>()
                                    .split(',')
                                    .map(str::to_string)
                                    .collect::<Vec<String>>();

        if parts.len() < 2 {
            return Err("Invalid number of arguments".into());
        }
        
        let (socket_fd, socket_name ) = split_fd_parts(&parts[0]);

        Ok(SocketTransferAttrs {
            socket_fd: socket_fd,
            socket_name: socket_name,
            buffer: parts[1].to_string(),
            size:parts[2].parse::<i32>().unwrap(),
            flags: parts[3].to_string(),
            socket_addr: parts[4].to_string(),
            socket_len: parts[5].parse::<i32>().unwrap_or(0)
        })
    }   

    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}