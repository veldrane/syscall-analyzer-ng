use helpers::helpers::split_fd_parts;
use wrappers::parsers::Parsable;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::rc::Rc;


#[derive(Debug, Serialize,Deserialize)]
pub struct NetworkListenAttrs {
    socket_fd: String,
    socket_name: String,
    backlog: String,
}


#[typetag::serde]
impl Parsable for NetworkListenAttrs {
    fn parse(args: &str, _: Option<&str>) -> Result<Self, String> {

        let parts: Vec<String> = args
                                    .chars()
                                    .filter(|&c| !r#""\"? "#.contains(c))
                                    .collect::<String>()
                                    .split(',')
                                    .map(str::to_string)
                                    .collect::<Vec<String>>();

        if parts.len() != 2 {
            return Err("Invalid number of arguments".into());
        }
        let (socket_fd, socket_name ) = split_fd_parts(&parts[0]);

        Ok(NetworkListenAttrs {
            socket_fd: socket_fd.to_string(),
            socket_name: socket_name,
            backlog: parts[1].to_string(),
        })
    }
    
    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }  
}