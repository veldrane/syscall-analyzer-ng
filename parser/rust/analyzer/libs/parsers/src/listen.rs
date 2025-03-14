use helpers::helpers::split_fd_parts;
use registry::registry::Parsable;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize,Deserialize)]
pub struct ListenArgs {
    socket_fd: String,
    socket_name: String,
    backlog: String,
}

#[derive(Debug, Serialize,Deserialize)]
#[serde(transparent)]
pub struct ListenArgsWrapper(ListenArgs);

#[typetag::serde]
impl Parsable for ListenArgs {
    fn parse(input: &str) -> Result<Self, String> {

        let parts: Vec<String> = input
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

        Ok(ListenArgs {
            socket_fd: socket_fd.to_string(),
            socket_name: socket_name,
            backlog: parts[1].to_string(),
        })
    }   
}