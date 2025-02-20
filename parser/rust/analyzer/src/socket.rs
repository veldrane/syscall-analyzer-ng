use serde::{Serialize,Deserialize};

use crate::registry::SyscallArguments;

#[derive(Debug, Serialize,Deserialize)]
pub struct SocketArgs {
    domain: String,
    socket_type: String,
    protocol: String,
}

#[typetag::serde]
impl SyscallArguments for SocketArgs {
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