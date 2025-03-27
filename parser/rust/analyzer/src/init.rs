use std::collections::HashMap;
use registry::registry::Register;
use wrappers::parsers::parser_wrapper;
use parsers::*;

pub fn init_registry() -> HashMap<String, Register> {
    
    return HashMap::from([
        ("mmap".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<mmap::MmapArgs>),
                trackers: None,      
        }),
        ("mprotect".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<mprotect::MprotectArgs>),
                trackers: None,
        }),
        ("munmap".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<munmap::MunmapArgs>), 
                trackers: None,
        }),
        ("access".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<access::AccessArgs>),
                trackers: None,
        }),
        ("openat".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<openat::OpenatArguments>),
                trackers: None,   
        }),
        ("dup2".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<dup2::Dup2Args>),
                trackers: None,
        }),
        ("socket".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<socket::SocketArgs>),
                trackers: None,
        }),
        ("setsockopt".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<socket::SocketArgs>),
                trackers: None,
        }),
        ("getsockopt".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<socket::SocketArgs>),
                trackers: None,
        }),
        ("accept".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<network::NetworkArgs>),
                trackers: None,
        }),
        ("accept4".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<network::Accept4Args>),
                trackers: None,
        }),
        ("connect".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<network::NetworkArgs>),
                trackers: None,
        }),
        ("bind".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<network::NetworkArgs>),
                trackers: None,
        }),
        ("listen".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<listen::ListenArgs>),
                trackers: None,   
        }),
        ("fcntl".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<fcntl::FcntlArgs>),
                trackers: None,   
        }),
        ("pread64".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<readwrite::ReadWriteArgs>),
                trackers: None,   
        }),
        ("pwrite64".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<readwrite::ReadWriteArgs>),
                trackers: None,   
        }),
        ("write".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<readwrite::ReadWriteArgs>),
                trackers: None,   
        }),
        ("read".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<readwrite::ReadWriteArgs>),
                trackers: None,   
        }),
        ("sendto".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<recvsend::RecvSendArgs>),
                trackers: None,   
        }),
        ("recvfrom".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<recvsend::RecvSendArgs>),
                trackers: None,   
        }),
        ("clone".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<clone::CloneArgs>),
                trackers: None,   
        }),
        ("close".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<close::CloseArgs>),
                trackers: None,   
        }),
        ("sendmsg".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<messages::MessagesArgs>),
                trackers: None,   
        }),
        ("recvmsg".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<messages::MessagesArgs>),
                trackers: None,   
        }),
        ("epoll_create".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<epoll_create::EpollCreateArgs>),
                trackers: None,   
        }),
        ("epoll_create1".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<epoll_create::EpollCreate1Args>),
                trackers: None,
        }),
        ("epoll_ctl".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<epoll_ctl::EpollCtlArgs>),
                trackers: None,   
        }),
        ("epoll_wait".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<epoll_wait::EpollWaitArgs>),
                trackers: None,   
        }),
        ("epoll_pwait2".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<epoll_wait::EpollPwait2Args>),
                trackers: None,
        }),
        ("eventfd2".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<eventfd::Eventfd2Args>),
                trackers: None,   
        })
    ]);
}
