use std::collections::HashMap;

use registry::registry::{Register, args_wrapper, returns_wrapper};
use parsers::{mmap, munmap, openat, socket, accept, listen, fcntl, pread64, sendto, clone, close, sendmsg, epoll_create};

pub fn init_registry() -> HashMap<String, Register> {
    
    return HashMap::from([
        ("mmap".to_string(), 
            Register { 
                arguments: Box::new(args_wrapper::<mmap::MmapArguments>), 
                returns: None,
        }),
        ("munmap".to_string(), 
            Register { 
                arguments: Box::new(args_wrapper::<munmap::MunmapArguments>), 
                returns: None,
        }),
        ("openat".to_string(), 
            Register { 
                arguments: Box::new(args_wrapper::<openat::OpenatArguments>), 
                returns: None,
        }),
        ("socket".to_string(), 
            Register { 
                arguments: Box::new(args_wrapper::<socket::SocketArgs>), 
                returns: None,
        }),
        ("accept".to_string(), 
            Register { 
                arguments: Box::new(args_wrapper::<accept::AcceptArgs>), 
                returns: None,
        }),
        ("accept4".to_string(), 
            Register { 
                arguments: Box::new(args_wrapper::<accept::AcceptArgs>), 
                returns: None,
        }),
        ("connect".to_string(), 
            Register { 
                arguments: Box::new(args_wrapper::<accept::AcceptArgs>), 
                returns: None,
        }),
        ("bind".to_string(), 
            Register { 
                arguments: Box::new(args_wrapper::<accept::AcceptArgs>), 
                returns: None,
        }),
        ("listen".to_string(), 
            Register { 
                arguments: Box::new(args_wrapper::<listen::ListenArgs>), 
                returns: None,
        }),
        ("fcntl".to_string(), 
            Register { 
                arguments: Box::new(args_wrapper::<fcntl::FcntlArgs>), 
                returns: None,
        }),
        ("pread64".to_string(), 
            Register { 
                arguments: Box::new(args_wrapper::<pread64::ReadArgs>), 
                returns: None,
        }),
        ("sendto".to_string(), 
            Register { 
                arguments: Box::new(args_wrapper::<sendto::SendtoArgs>), 
                returns: None,
        }),
        ("recvfrom".to_string(), 
            Register { 
                arguments: Box::new(args_wrapper::<sendto::SendtoArgs>), 
                returns: None,
        }),
        ("clone".to_string(), 
            Register { 
                arguments: Box::new(args_wrapper::<clone::CloneArgs>), 
                returns: Some(Box::new(returns_wrapper::<clone::CloneReturns>)),
        }),
        ("close".to_string(), 
            Register { 
                arguments: Box::new(args_wrapper::<close::CloseArgs>), 
                returns: None,
        }),
        ("sendmsg".to_string(), 
            Register { 
                arguments: Box::new(args_wrapper::<sendmsg::SendmsgArgs>), 
                returns: None,
        }),
        ("recvmsg".to_string(), 
            Register { 
                arguments: Box::new(args_wrapper::<sendmsg::SendmsgArgs>), 
                returns: None,
        }),
        ("epoll_create".to_string(), 
            Register { 
                arguments: Box::new(args_wrapper::<epoll_create::EpollCreateArgs>), 
                returns: None,
        }),
        ("epoll_create1".to_string(), 
            Register { 
                arguments: Box::new(args_wrapper::<epoll_create::EpollCreateArgs>), 
                returns: None,
        })
    ]);
}
