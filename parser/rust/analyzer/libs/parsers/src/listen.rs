use helpers::helpers::split_fd_parts;
use registry::registry::SyscallArguments;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize,Deserialize)]
pub struct ListenArgs {
    sockfd: String,
    sock_name: String,
    backlog: String,
}

#[derive(Debug, Serialize,Deserialize)]
#[serde(transparent)]
pub struct ListenArgsWrapper(ListenArgs);

#[typetag::serde]
impl SyscallArguments for ListenArgs {
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
        let (sockfd, sock_name ) = split_fd_parts(&parts[0]);

        Ok(ListenArgs {
            sockfd: sockfd.to_string(),
            sock_name: sock_name,
            backlog: parts[1].to_string(),
        })
    }   
}