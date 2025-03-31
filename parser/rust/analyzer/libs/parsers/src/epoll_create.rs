use wrappers::parsers::Parsable;
use serde::{Deserialize, Serialize};
use helpers::helpers::split_fd_parts;
use std::any::Any;
use std::rc::Rc;


#[derive(Debug, Serialize,Deserialize)]
pub struct EpollCreateArgs {
    size: i32,
    epoll_fd: i32,
    epoll_name: String,
}


#[typetag::serde]
impl Parsable for EpollCreateArgs {
    fn parse(args: &str, result: Option<&str>) -> Result<Self, String> {

        let size = args.parse::<i32>().map_err(|e| e.to_string())?;

        let (epoll_fd, epoll_name) = match result {
            Some(r) => split_fd_parts(&r),
            None => (0, "".to_string())
        };

        Ok(EpollCreateArgs {
            size: size,
            epoll_fd: epoll_fd,
            epoll_name: epoll_name,
        })
    }
    
    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    } 
}

#[derive(Debug, Serialize,Deserialize)]
pub struct EpollCreate1Args {
    flags: String,
    epoll_fd: i32,
    epoll_name: String,
}


#[typetag::serde]
impl Parsable for EpollCreate1Args {
    fn parse(args: &str, result: Option<&str>) -> Result<Self, String> {

        let (epoll_fd, epoll_name) = match result {
            Some(r) => split_fd_parts(&r),
            None => (0, "".to_string())
        };

        Ok(EpollCreate1Args {
            flags: args.to_string(),
            epoll_fd: epoll_fd,
            epoll_name: epoll_name,
        })
    }

    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }  
}