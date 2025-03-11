use std::collections::HashMap;

use registry::registry::{Register, parser_wrapper};
use parsers::{mmap, munmap, openat, socket, accept, listen, fcntl, pread64, sendto, clone, close, sendmsg, epoll_create};

pub fn init_registry() -> HashMap<String, Register> {
    
    return HashMap::from([
        ("mmap".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<mmap::MmapArguments>), 
                returns: None,
        }),
        ("munmap".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<munmap::MunmapArguments>), 
                returns: None,
        }),
        ("openat".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<openat::OpenatArguments>), 
                returns: Some(Box::new(parser_wrapper::<openat::OpenatResults>)),
        }),
        ("socket".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<socket::SocketArgs>), 
                returns: Some(Box::new(parser_wrapper::<socket::SocketResults>)),
        }),
        ("accept".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<accept::AcceptArgs>), 
                returns: None,
        }),
        ("accept4".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<accept::AcceptArgs>), 
                returns: None,
        }),
        ("connect".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<accept::AcceptArgs>), 
                returns: None,
        }),
        ("bind".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<accept::AcceptArgs>), 
                returns: None,
        }),
        ("listen".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<listen::ListenArgs>), 
                returns: None,
        }),
        ("fcntl".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<fcntl::FcntlArgs>), 
                returns: None,
        }),
        ("pread64".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<pread64::ReadArgs>), 
                returns: Some(Box::new(parser_wrapper::<pread64::ReadResults>)),
        }),
        ("pwrite64".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<pread64::ReadArgs>), 
                returns: Some(Box::new(parser_wrapper::<pread64::ReadResults>)),
        }),
        ("write".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<pread64::ReadArgs>), 
                returns: Some(Box::new(parser_wrapper::<pread64::ReadResults>)),
        }),
        ("read".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<pread64::ReadArgs>), 
                returns: Some(Box::new(parser_wrapper::<pread64::ReadResults>)),
        }),
        ("sendto".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<sendto::SendtoArgs>), 
                returns: None,
        }),
        ("recvfrom".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<sendto::SendtoArgs>), 
                returns: None,
        }),
        ("clone".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<clone::CloneArgs>), 
                returns: Some(Box::new(parser_wrapper::<clone::CloneResults>)),
        }),
        ("close".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<close::CloseArgs>), 
                returns: None,
        }),
        ("sendmsg".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<sendmsg::SendmsgArgs>), 
                returns: None,
        }),
        ("recvmsg".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<sendmsg::SendmsgArgs>), 
                returns: None,
        }),
        ("epoll_create".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<epoll_create::EpollCreateArgs>), 
                returns: Some(Box::new(parser_wrapper::<epoll_create::EpollCreateResults>)),
        }),
        ("epoll_create1".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<epoll_create::EpollCreate1Args>), 
                returns: Some(Box::new(parser_wrapper::<epoll_create::EpollCreateResults>)),
        })
    ]);
}
