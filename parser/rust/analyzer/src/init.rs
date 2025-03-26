use std::collections::HashMap;
use registry::registry::Register;
use wrappers::parsers::parser_wrapper;
use parsers::*;

pub fn init_registry() -> HashMap<String, Register> {
    
    return HashMap::from([
        ("mmap".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<mmap::MmapArgs>),         
        }),
        ("mprotect".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<mprotect::MprotectArgs>), 
        }),
        ("munmap".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<munmap::MunmapArgs>), 
        }),
        ("access".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<access::AccessArgs>), 
        }),
        ("openat".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<openat::OpenatArguments>),
        }),
        ("dup2".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<dup2::Dup2Args>),
        }),
        ("socket".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<socket::SocketArgs>),
        }),
        ("setsockopt".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<socket::SocketArgs>),
        }),
        ("getsockopt".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<socket::SocketArgs>), 
        }),
        ("accept".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<network::NetworkArgs>),
        }),
        ("accept4".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<network::Accept4Args>),
        }),
        ("connect".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<network::NetworkArgs>), 
        }),
        ("bind".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<network::NetworkArgs>), 
        }),
        ("listen".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<listen::ListenArgs>), 
        }),
        ("fcntl".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<fcntl::FcntlArgs>),
        }),
        ("pread64".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<readwrite::ReadWriteArgs>),
        }),
        ("pwrite64".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<readwrite::ReadWriteArgs>), 
        }),
        ("write".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<readwrite::ReadWriteArgs>),
        }),
        ("read".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<readwrite::ReadWriteArgs>), 
        }),
        ("sendto".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<recvsend::RecvSendArgs>),
        }),
        ("recvfrom".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<recvsend::RecvSendArgs>),
        }),
        ("clone".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<clone::CloneArgs>),
        }),
        ("close".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<close::CloseArgs>), 
               
        }),
        ("sendmsg".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<messages::MessagesArgs>), 
               
        }),
        ("recvmsg".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<messages::MessagesArgs>),
        }),
        ("epoll_create".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<epoll_create::EpollCreateArgs>),
        }),
        ("epoll_create1".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<epoll_create::EpollCreate1Args>),
        }),
        ("epoll_ctl".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<epoll_ctl::EpollCtlArgs>),
        }),
        ("epoll_wait".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<epoll_wait::EpollWaitArgs>), 
        }),
        ("epoll_pwait2".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<epoll_wait::EpollPwait2Args>), 
        }),
        ("eventfd2".to_string(), 
            Register { 
                attributes: Box::new(parser_wrapper::<eventfd::Eventfd2Args>),
        })
    ]);
}
