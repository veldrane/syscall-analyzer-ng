use helpers::helpers::split_fd_parts;
use registry::registry::Parsable;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize,Deserialize)]
pub struct SendtoArgs {
    socket_fd: i32,
    socket_name: String,
    buffer: String,
    size: i32,
    flags: String,
    sock_addr: String,
    sock_len: String,
}


#[typetag::serde]
impl Parsable for SendtoArgs {
    fn parse(input: &str) -> Result<Self, String> {

        let parts: Vec<String> = input
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

        //let opt_arg = if let Some(opt) = parts.get(2) {
        //    opt.to_string()
        //} else {
        //    "".to_string()
        //};

        Ok(SendtoArgs {
            socket_fd: socket_fd,
            socket_name: socket_name,
            buffer: parts[1].to_string(),
            size:parts[2].parse::<i32>().unwrap(),
            flags: parts[3].to_string(),
            sock_addr: parts[4].to_string(),
            sock_len: parts[5].to_string()
        })
    }   
}