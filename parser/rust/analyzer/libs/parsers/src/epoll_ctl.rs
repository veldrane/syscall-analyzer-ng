use registry::registry::Parsable;
use serde::{Deserialize, Serialize};
use helpers::helpers::split_fd_parts;
use serde_with::skip_serializing_none;
use regex::Regex;

const EPOLL_EVENT: &str = r".*\,\s\{(?P<epoll_event>.*)\}";


#[skip_serializing_none]
#[derive(Debug, Serialize,Deserialize, Default)]
pub struct EpollCtlArgs {
    epoll_fd: i32,
    epoll_name: String,
    epoll_opetation: String,
    epoll_event: Option<String>,
    fd: Option<i32>,
    name: Option<String>,
    socket_fd: Option<i32>,
    socket_name: Option<String>,
    timer_fd: Option<i32>,
    timer_name: Option<String>,
    event_fd: Option<i32>,
    event_name: Option<String>,
}


#[typetag::serde]
impl Parsable for EpollCtlArgs {
    fn parse(input: &str) -> Result<Self, String> {


        let re = Regex::new(EPOLL_EVENT).map_err(|e| e.to_string())?;

        //let caps = re.captures(&input).ok_or(input.to_string())?;

        let mut epoll_ctl_args = EpollCtlArgs::default();

        let parts: Vec<String> = input
                                    .chars()
                                    .filter(|&c| !r#""\"? "#.contains(c))
                                    .collect::<String>()
                                    .split(',')
                                    .map(str::to_string)
                                    .collect::<Vec<String>>();

        if parts.len() < 4 {
            return Err("Invalid number of arguments".into());
        }

        match parts[3].as_str() {
            "NULL" => {
                epoll_ctl_args.epoll_event = None;
            },
            _ => {
                let caps = re.captures(&input).ok_or(input.to_string())?;
                epoll_ctl_args.epoll_event = Some(caps["epoll_event"].to_string());
            }
        }

        (epoll_ctl_args.epoll_fd, epoll_ctl_args.epoll_name ) = split_fd_parts(&parts[0]);
        epoll_ctl_args.epoll_opetation = parts[1].to_string();
        
        let (fd, name ) = split_fd_parts(&parts[2]);

        match name.as_str() {
            s if s.contains("socket:") => {
                epoll_ctl_args.socket_name = Some(name);
                epoll_ctl_args.socket_fd = Some(fd);
            },
            s if s.contains("anon_inode:[timerfd]") => {
                epoll_ctl_args.timer_fd = Some(fd);
                epoll_ctl_args.timer_name = Some(name);
            },
            s if s.contains("anon_inode:[eventfd]") => {
                epoll_ctl_args.event_fd = Some(fd);
                epoll_ctl_args.event_name = Some(name);
            },
            _ => {
                epoll_ctl_args.fd = Some(fd);
                epoll_ctl_args.name = Some(name);
            }
        };

        Ok(epoll_ctl_args)
    }   
}


