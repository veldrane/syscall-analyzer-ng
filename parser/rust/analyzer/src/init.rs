use std::collections::HashMap;

use registry::registry::{Register, parser_wrapper};
use parsers::{mmap, munmap, openat, socket, accept, listen, fcntl, pread64, sendto, clone, close, sendmsg, epoll_create};

pub fn init_registry() -> HashMap<String, Register> {
    
    return HashMap::from([
        ("mmap".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<mmap::MmapArguments>), 
                results: None,
        }),
        ("munmap".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<munmap::MunmapArguments>), 
                results: None,
        }),
        ("openat".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<openat::OpenatArguments>), 
                results: Some(Box::new(parser_wrapper::<openat::OpenatResults>)),
        }),
        ("socket".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<socket::SocketArgs>), 
                results: Some(Box::new(parser_wrapper::<socket::SocketResults>)),
        }),
        ("accept".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<accept::AcceptArgs>), 
                results: None,
        }),
        ("accept4".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<accept::AcceptArgs>), 
                results: None,
        }),
        ("connect".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<accept::AcceptArgs>), 
                results: None,
        }),
        ("bind".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<accept::AcceptArgs>), 
                results: None,
        }),
        ("listen".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<listen::ListenArgs>), 
                results: None,
        }),
        ("fcntl".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<fcntl::FcntlArgs>), 
                results: None,
        }),
        ("pread64".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<pread64::ReadArgs>), 
                results: Some(Box::new(parser_wrapper::<pread64::ReadResults>)),
        }),
        ("pwrite64".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<pread64::ReadArgs>), 
                results: Some(Box::new(parser_wrapper::<pread64::ReadResults>)),
        }),
        ("write".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<pread64::ReadArgs>), 
                results: Some(Box::new(parser_wrapper::<pread64::ReadResults>)),
        }),
        ("read".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<pread64::ReadArgs>), 
                results: Some(Box::new(parser_wrapper::<pread64::ReadResults>)),
        }),
        ("sendto".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<sendto::SendtoArgs>), 
                results: None,
        }),
        ("recvfrom".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<sendto::SendtoArgs>), 
                results: None,
        }),
        ("clone".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<clone::CloneArgs>), 
                results: Some(Box::new(parser_wrapper::<clone::CloneResults>)),
        }),
        ("close".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<close::CloseArgs>), 
                results: None,
        }),
        ("sendmsg".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<sendmsg::SendmsgArgs>), 
                results: None,
        }),
        ("recvmsg".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<sendmsg::SendmsgArgs>), 
                results: None,
        }),
        ("epoll_create".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<epoll_create::EpollCreateArgs>), 
                results: Some(Box::new(parser_wrapper::<epoll_create::EpollCreateResults>)),
        }),
        ("epoll_create1".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<epoll_create::EpollCreate1Args>), 
                results: Some(Box::new(parser_wrapper::<epoll_create::EpollCreateResults>)),
        })
    ]);
}
