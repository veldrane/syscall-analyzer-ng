use std::collections::HashMap;

use registry::registry::{Register, parser_wrapper};
use parsers::{mmap, munmap, mprotect, access, openat, socket, network, listen, fcntl, readwrite, recvsend, clone, close, messages, epoll_create, dup2};

pub fn init_registry() -> HashMap<String, Register> {
    
    return HashMap::from([
        ("mmap".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<mmap::MmapArguments>), 
                results: None,
        }),
        ("mprotect".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<mprotect::MprotectArgs>), 
                results: None,
        }),
        ("munmap".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<munmap::MunmapArguments>), 
                results: None,
        }),
        ("access".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<access::AccessArgs>), 
                results: None,
        }),
        ("openat".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<openat::OpenatArguments>), 
                results: Some(Box::new(parser_wrapper::<openat::OpenatResults>)),
        }),
        ("dup2".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<dup2::Dup2Args>), 
                results: Some(Box::new(parser_wrapper::<dup2::Dup2Results>)),
        }),
        ("socket".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<socket::SocketArgs>), 
                results: Some(Box::new(parser_wrapper::<socket::SocketResults>)),
        }),
        ("setsockopt".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<socket::SocketOptArgs>), 
                results: None,
        }),
        ("getsockopt".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<socket::SocketOptArgs>), 
                results: None,
        }),
        ("accept".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<network::NetworkArgs>), 
                results: None,
        }),
        ("accept4".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<network::Accept4Args>), 
                results: Some(Box::new(parser_wrapper::<network::Accept4Results>)),
        }),
        ("connect".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<network::NetworkArgs>), 
                results: None,
        }),
        ("bind".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<network::NetworkArgs>), 
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
                arguments: Box::new(parser_wrapper::<readwrite::ReadWriteArgs>), 
                results: Some(Box::new(parser_wrapper::<readwrite::ReadWriteResults>)),
        }),
        ("pwrite64".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<readwrite::ReadWriteArgs>), 
                results: Some(Box::new(parser_wrapper::<readwrite::ReadWriteResults>)),
        }),
        ("write".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<readwrite::ReadWriteArgs>), 
                results: Some(Box::new(parser_wrapper::<readwrite::ReadWriteResults>)),
        }),
        ("read".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<readwrite::ReadWriteArgs>), 
                results: Some(Box::new(parser_wrapper::<readwrite::ReadWriteResults>)),
        }),
        ("sendto".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<recvsend::RecvSend>), 
                results: None,
        }),
        ("recvfrom".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<recvsend::RecvSend>), 
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
                arguments: Box::new(parser_wrapper::<messages::MessagesArgs>), 
                results: None,
        }),
        ("recvmsg".to_string(), 
            Register { 
                arguments: Box::new(parser_wrapper::<messages::MessagesArgs>), 
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
