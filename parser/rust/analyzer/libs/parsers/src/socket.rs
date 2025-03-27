use serde::{Serialize,Deserialize};
use wrappers::parsers::Parsable;
use helpers::helpers::split_fd_parts;

#[derive(Debug, Serialize,Deserialize)]
pub struct SocketArgs {
    domain: String,
    socket_type: String,
    protocol: String,
    socket_fd: i32,
    socket_name: String,
}

#[typetag::serde]
impl Parsable for SocketArgs {
    fn parse(args: &str, result: Option<&str>) -> Result<Self, String> {
        
        let parts: Vec<String> = args
                                    .chars()
                                    .filter(|&c| !r#""\"? "#.contains(c))
                                    .collect::<String>()
                                    .split(',')
                                    .map(str::to_string)
                                    .collect::<Vec<String>>();
        if parts.len() != 3 {
            return Err("Invalid number of arguments".into());
        }

        let (socket_fd, socket_name ) = match result {
            Some(r) => split_fd_parts(&r),
            None => (0, "".to_string())
        };

        Ok(SocketArgs {
            socket_fd: socket_fd,
            socket_name: socket_name,
            domain: parts[0].to_string(),
            socket_type: parts[1].to_string(),
            protocol: parts[2].to_string()
        })
    }   
}