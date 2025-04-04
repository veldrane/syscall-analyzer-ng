use wrappers::parsers::Parsable;
use serde::{Deserialize, Serialize};
use helpers::helpers::split_fd_parts;
use regex::Regex;
use once_cell::sync::Lazy;
use std::any::Any;
use std::rc::Rc;

const EPOLL_WAIT_ARGS: &str = r".*\,\s\[\{(?P<epoll_event>.*)\}\]\,\s(?P<maxevents>.+)\,\s(?P<timeout>.+)";

const EPOLL_PWAIT2_ARGS: &str = r"(?P<epoll_descriptor>.+)\,\s\[(?P<epoll_event>.*)\]\,\s(?P<maxevents>\d+)\,\s\{(?P<timespec>.+)\}\,\s(?P<sigmask>.+)\,\s.+";

static RE_EPOLL_WAIT_ARGS: Lazy<Regex> = Lazy::new(|| Regex::new(EPOLL_WAIT_ARGS).unwrap());
static RE_EPOLL_PWAIT2_ARGS: Lazy<Regex> = Lazy::new(|| Regex::new(EPOLL_PWAIT2_ARGS).unwrap());


#[derive(Debug, Serialize,Deserialize, Default)]
pub struct EpollWaitAttrs {
    epoll_fd: i32,
    epoll_name: String,
    epoll_event: Option<String>,
    maxevents: i32,
    timeout: i32,
}



#[typetag::serde]
impl Parsable for EpollWaitAttrs {
    fn parse(args: &str, _: Option<&str>) -> Result<Self, String> {

        //let re = Regex::new(EPOLL_WAIT_ARGS).map_err(|e| e.to_string())?;
        let mut epoll_wait_args = EpollWaitAttrs::default();

        let parts: Vec<String> = args
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

        let caps = RE_EPOLL_WAIT_ARGS.captures(&args).ok_or(args.to_string())?;

        epoll_wait_args.epoll_event = Some(caps["epoll_event"].parse::<String>().unwrap());
        epoll_wait_args.maxevents = caps["maxevents"].parse::<i32>().unwrap();
        epoll_wait_args.timeout = caps["timeout"].parse::<i32>().unwrap();
        Ok(epoll_wait_args)
    }

    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }

}

#[derive(Debug, Serialize,Deserialize, Default)]
pub struct EpollPwait2Args {
    epoll_fd: i32,
    epoll_name: String,
    epoll_event: Option<String>,
    maxevents: i32,
    timespec: String,
    sigmask: String,
}

#[typetag::serde]
impl Parsable for EpollPwait2Args {
    fn parse(args: &str, _: Option<&str>) -> Result<Self, String> {

        let mut epoll_pwait2_args = EpollPwait2Args::default();


        let caps = RE_EPOLL_PWAIT2_ARGS.captures(&args).ok_or(args.to_string())?;

        (epoll_pwait2_args.epoll_fd, epoll_pwait2_args.epoll_name ) = split_fd_parts(&caps["epoll_descriptor"]);



        epoll_pwait2_args.epoll_event = Some(caps["epoll_event"].parse::<String>().unwrap());
        epoll_pwait2_args.maxevents = caps["maxevents"].parse::<i32>().unwrap();
        epoll_pwait2_args.timespec = caps["timespec"].parse::<String>().unwrap();
        epoll_pwait2_args.sigmask = caps["sigmask"].parse::<String>().unwrap();
        Ok(epoll_pwait2_args)
    }

    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
    self
    }  
}