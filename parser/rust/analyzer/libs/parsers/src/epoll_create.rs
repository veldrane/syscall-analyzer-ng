use registry::registry::Parsable;
use serde::{Deserialize, Serialize};
use helpers::helpers::split_fd_parts;


#[derive(Debug, Serialize,Deserialize)]
pub struct EpollCreateArgs {
    size: i32,
}


#[derive(Debug, Serialize,Deserialize)]
pub struct EpollCreateResults {
    epoll_fd: i32,
    epoll_name: String,
}


#[typetag::serde]
impl Parsable for EpollCreateResults {

    fn parse(input: &str) -> Result<Self, String> {

        let (fd, file_name ) = split_fd_parts(input);

        Ok(EpollCreateResults {
            epoll_fd: fd,
            epoll_name: file_name,
        })
    }
}

#[typetag::serde]
impl Parsable for EpollCreateArgs {
    fn parse(input: &str) -> Result<Self, String> {

        let size = input.parse::<i32>().map_err(|e| e.to_string())?;

        Ok(EpollCreateArgs {
            size: size,
        })
    }   
}

#[derive(Debug, Serialize,Deserialize)]
pub struct EpollCreate1Args {
    flags: String,
}


#[typetag::serde]
impl Parsable for EpollCreate1Args {
    fn parse(input: &str) -> Result<Self, String> {

        Ok(EpollCreate1Args {
            flags: input.to_string(),
        })
    }   
}