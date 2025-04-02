use std::collections::HashMap;
use registry::registry::Register;
use wrappers::{parsers::parser_wrapper, trackers::tracker_wrapper};
use parsers::*;
use std::rc::Rc;

pub fn init_registry() -> HashMap<String, Register> {
    
    return HashMap::from([
        ("mmap".to_string(), 
            Register { 
                attributes: Rc::new(parser_wrapper::<mmap::MmapAttrs>),
                trackers: None,      
        }),
        ("mprotect".to_string(), 
            Register { 
                attributes: Rc::new(parser_wrapper::<mprotect::MprotectAttrs>),
                trackers: None,
        }),
        ("munmap".to_string(), 
            Register { 
                attributes: Rc::new(parser_wrapper::<munmap::MunmapAttrs>), 
                trackers: None,
        }),
        ("access".to_string(), 
            Register { 
                attributes: Rc::new(parser_wrapper::<access::FileAccessAttrs>),
                trackers: None,
        }),
        ("openat".to_string(), 
            Register { 
                attributes: Rc::new(parser_wrapper::<openat::OpenAtAttrs>),
                trackers: Some(Box::new(tracker_wrapper::<openat::OpenatTrack>))
        }),
        ("dup2".to_string(), 
            Register { 
                attributes: Rc::new(parser_wrapper::<dup2::DuplicationAttrs>),
                trackers: None,
        }),
        ("socket".to_string(), 
            Register { 
                attributes: Rc::new(parser_wrapper::<socket::SocketArgs>),
                trackers: Some(Box::new(tracker_wrapper::<socket::SocketTrack>))
        }),
        ("setsockopt".to_string(), 
            Register { 
                attributes: Rc::new(parser_wrapper::<socket::SocketArgs>),
                trackers: None,
        }),
        ("getsockopt".to_string(), 
            Register { 
                attributes: Rc::new(parser_wrapper::<socket::SocketArgs>),
                trackers: None,
        }),
        ("accept".to_string(), 
            Register { 
                attributes: Rc::new(parser_wrapper::<network::NetworkSocketAttrs>),
                trackers: None,
        }),
        ("accept4".to_string(), 
            Register { 
                attributes: Rc::new(parser_wrapper::<network::Accept4Attrs>),
                trackers: None,
        }),
        ("connect".to_string(), 
            Register { 
                attributes: Rc::new(parser_wrapper::<network::NetworkSocketAttrs>),
                trackers: None,
        }),
        ("bind".to_string(), 
            Register { 
                attributes: Rc::new(parser_wrapper::<network::NetworkSocketAttrs>),
                trackers: None,
        }),
        ("listen".to_string(), 
            Register { 
                attributes: Rc::new(parser_wrapper::<listen::NetworkListenAttrs>),
                trackers: None,   
        }),
        ("fcntl".to_string(), 
            Register { 
                attributes: Rc::new(parser_wrapper::<fcntl::FileControlAttrs>),
                trackers: None,   
        }),
        ("pread64".to_string(), 
            Register { 
                attributes: Rc::new(parser_wrapper::<read_write::IORequestAttrs>),
                trackers: Some(Box::new(tracker_wrapper::<read_write::ReadWriteTrack>))  
        }),
        ("pwrite64".to_string(), 
            Register { 
                attributes: Rc::new(parser_wrapper::<read_write::IORequestAttrs>),
                trackers: Some(Box::new(tracker_wrapper::<read_write::ReadWriteTrack>))  
        }),
        ("write".to_string(), 
            Register { 
                attributes: Rc::new(parser_wrapper::<read_write::IORequestAttrs>),
                trackers: Some(Box::new(tracker_wrapper::<read_write::ReadWriteTrack>)),   
        }),
        ("read".to_string(), 
            Register { 
                attributes: Rc::new(parser_wrapper::<read_write::IORequestAttrs>),
                trackers: Some(Box::new(tracker_wrapper::<read_write::ReadWriteTrack>)),   
        }),
        ("sendto".to_string(), 
            Register { 
                attributes: Rc::new(parser_wrapper::<recv_send::SocketTransferAttrs>),
                trackers: None,   
        }),
        ("recvfrom".to_string(), 
            Register { 
                attributes: Rc::new(parser_wrapper::<recv_send::SocketTransferAttrs>),
                trackers: None,   
        }),
        ("clone".to_string(), 
            Register { 
                attributes: Rc::new(parser_wrapper::<clone::CloneAttrs>),
                trackers: None,   
        }),
        ("close".to_string(), 
            Register { 
                attributes: Rc::new(parser_wrapper::<close::CloseFileAttrs>),
                trackers: Some(Box::new(tracker_wrapper::<close::CloseTrack>))   
        }),
        ("sendmsg".to_string(), 
            Register { 
                attributes: Rc::new(parser_wrapper::<messages::SocketMessageAttrs>),
                trackers: None,   
        }),
        ("recvmsg".to_string(), 
            Register { 
                attributes: Rc::new(parser_wrapper::<messages::SocketMessageAttrs>),
                trackers: None,   
        }),
        ("epoll_create".to_string(), 
            Register { 
                attributes: Rc::new(parser_wrapper::<epoll_create::EpollCreationAttrs>),
                trackers: None,   
        }),
        ("epoll_create1".to_string(), 
            Register { 
                attributes: Rc::new(parser_wrapper::<epoll_create::EpollCreate1Attrs>),
                trackers: None,
        }),
        ("epoll_ctl".to_string(), 
            Register { 
                attributes: Rc::new(parser_wrapper::<epoll_ctl::EpollControlAttrs>),
                trackers: None,   
        }),
        ("epoll_wait".to_string(), 
            Register { 
                attributes: Rc::new(parser_wrapper::<epoll_wait::EpollWaitAttrs>),
                trackers: None,   
        }),
        ("epoll_pwait2".to_string(), 
            Register { 
                attributes: Rc::new(parser_wrapper::<epoll_wait::EpollPwait2Args>),
                trackers: None,
        }),
        ("eventfd2".to_string(), 
            Register { 
                attributes: Rc::new(parser_wrapper::<eventfd::Eventfd2Attrs>),
                trackers: None,   
        })
    ]);
}
