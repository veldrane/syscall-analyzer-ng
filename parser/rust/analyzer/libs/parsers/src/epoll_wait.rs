use registry::registry::Parsable;
use serde::{Deserialize, Serialize};
use helpers::helpers::split_fd_parts;
use regex::Regex;

const EPOLL_WAIT_ARGS: &str = r".*\,\s\[\{(?P<epoll_event>.*)\}\]\,\s(?P<maxevents>.+)\,\s(?P<timeout>.+)";


#[derive(Debug, Serialize,Deserialize, Default)]
pub struct EpollWaitArgs {
    epoll_fd: i32,
    epoll_name: String,
    epoll_event: Option<String>,
    maxevents: i32,
    timeout: i32,
}



#[typetag::serde]
impl Parsable for EpollWaitArgs {
    fn parse(input: &str) -> Result<Self, String> {

        let re = Regex::new(EPOLL_WAIT_ARGS).map_err(|e| e.to_string())?;
        let mut epoll_wait_args = EpollWaitArgs::default();

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

        (epoll_wait_args.epoll_fd, epoll_wait_args.epoll_name ) = split_fd_parts(&parts[0]);

        let caps = re.captures(&input).ok_or(input.to_string()).unwrap();

        epoll_wait_args.epoll_event = Some(caps["epoll_event"].parse::<String>().unwrap());
        epoll_wait_args.maxevents = caps["maxevents"].parse::<i32>().unwrap();
        epoll_wait_args.timeout = caps["timeout"].parse::<i32>().unwrap();
        Ok(epoll_wait_args)
    }   
}

#[derive(Debug, Serialize,Deserialize)]
pub struct EpollWait1Args {
    flags: String,
}


#[typetag::serde]
impl Parsable for EpollWait1Args {
    fn parse(input: &str) -> Result<Self, String> {

        Ok(EpollWait1Args {
            flags: input.to_string(),
        })
    }   
}