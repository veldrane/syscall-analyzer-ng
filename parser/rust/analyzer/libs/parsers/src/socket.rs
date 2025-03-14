use serde::{Serialize,Deserialize};
use registry::registry::Parsable;
use helpers::helpers::split_fd_parts;

#[derive(Debug, Serialize,Deserialize)]
pub struct SocketArgs {
    domain: String,
    socket_type: String,
    protocol: String,
}


#[typetag::serde]
impl Parsable for SocketArgs {
    fn parse(input: &str) -> Result<Self, String> {
        
        let parts: Vec<String> = input
                                    .chars()
                                    .filter(|&c| !r#""\"? "#.contains(c))
                                    .collect::<String>()
                                    .split(',')
                                    .map(str::to_string)
                                    .collect::<Vec<String>>();
        if parts.len() != 3 {
            return Err("Invalid number of arguments".into());
        }
        Ok(SocketArgs {
            domain: parts[0].to_string(),
            socket_type: parts[1].to_string(),
            protocol: parts[2].to_string()
        })
    }   
}


#[derive(Debug, Serialize,Deserialize)]
pub struct SocketResults {
    pub socket_fd: i32,
    pub socket_name: String,
}

#[typetag::serde]
impl Parsable for SocketResults {
    fn parse(input: &str) -> Result<Self, String> {
        
        let parts: Vec<&str> = input
                                    .split(' ')
                                    .collect();


        if parts[0] == "-1" {
            return Err("Error opening file".into());
        }

        let (socket_fd, socket_name) = split_fd_parts(&parts[0]);

        Ok(SocketResults {
            socket_fd: socket_fd,
            socket_name: socket_name,
        })
    }
}

#[derive(Debug, Serialize,Deserialize)]
pub struct SocketOptArgs {
    socket_fd: String,
    socket_name: String,
    level: String,
    operation: String,
    opt_val: String,
    socket_len: String,
}

#[typetag::serde]
impl Parsable for SocketOptArgs {

    fn parse(input: &str) -> Result<Self, String> {
        
        let parts: Vec<String> = input
                                    .chars()
                                    .filter(|&c| !r#""\"? "#.contains(c))
                                    .collect::<String>()
                                    .split(',')
                                    .map(str::to_string)
                                    .collect::<Vec<String>>();
        if parts.len() < 5 {
            return Err("Invalid number of arguments".into());
        }

        let (socket_fd, socket_name) = split_fd_parts(&parts[0]);

        Ok(SocketOptArgs {
            socket_fd: socket_fd.to_string(),
            socket_name: socket_name.to_string(),
            level: parts[1].to_string(),
            operation: parts[2].to_string(),
            opt_val: parts[3].to_string(),
            socket_len: parts[4].to_string()
        })
    }   
    
}