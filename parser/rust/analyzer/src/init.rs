use std::collections::HashMap;
use registry::registry::RegistryEntry;
use wrappers::{parsers::as_dyn_parser, trackers::as_dyn_tracker};
use parsers::*;
use std::rc::Rc;

pub fn build_registry() -> HashMap<String, RegistryEntry> {
    
    return HashMap::from([
        ("mmap".to_string(), 
            RegistryEntry { 
                attributes: Rc::new(as_dyn_parser::<mmap::MmapAttrs>),
                trackers: None,      
        }),
        ("mprotect".to_string(), 
            RegistryEntry { 
                attributes: Rc::new(as_dyn_parser::<mprotect::MprotectAttrs>),
                trackers: None,
        }),
        ("munmap".to_string(), 
            RegistryEntry { 
                attributes: Rc::new(as_dyn_parser::<munmap::MunmapAttrs>), 
                trackers: None,
        }),
        ("access".to_string(), 
            RegistryEntry { 
                attributes: Rc::new(as_dyn_parser::<access::FileAccessAttrs>),
                trackers: None,
        }),
        ("openat".to_string(), 
            RegistryEntry { 
                attributes: Rc::new(as_dyn_parser::<openat::OpenAtAttrs>),
                trackers: Some(Box::new(as_dyn_tracker::<openat::FileDescriptorTracker>))
        }),
        ("dup2".to_string(), 
            RegistryEntry { 
                attributes: Rc::new(as_dyn_parser::<dup2::DuplicationAttrs>),
                trackers: None,
        }),
        ("socket".to_string(), 
            RegistryEntry { 
                attributes: Rc::new(as_dyn_parser::<socket::NetworkSocketAttrs>),
                trackers: Some(Box::new(as_dyn_tracker::<socket::NetworkSocketTracker>))
        }),
        ("setsockopt".to_string(), 
            RegistryEntry { 
                attributes: Rc::new(as_dyn_parser::<socket::NetworkSocketAttrs>),
                trackers: None,
        }),
        ("getsockopt".to_string(), 
            RegistryEntry { 
                attributes: Rc::new(as_dyn_parser::<socket::NetworkSocketAttrs>),
                trackers: None,
        }),
        ("accept".to_string(), 
            RegistryEntry { 
                attributes: Rc::new(as_dyn_parser::<network::NetworkSocketAttrs>),
                trackers: None,
        }),
        ("accept4".to_string(), 
            RegistryEntry { 
                attributes: Rc::new(as_dyn_parser::<network::Accept4Attrs>),
                trackers: None,
        }),
        ("connect".to_string(), 
            RegistryEntry { 
                attributes: Rc::new(as_dyn_parser::<network::NetworkSocketAttrs>),
                trackers: None,
        }),
        ("bind".to_string(), 
            RegistryEntry { 
                attributes: Rc::new(as_dyn_parser::<network::NetworkSocketAttrs>),
                trackers: None,
        }),
        ("listen".to_string(), 
            RegistryEntry { 
                attributes: Rc::new(as_dyn_parser::<listen::NetworkListenAttrs>),
                trackers: None,   
        }),
        ("fcntl".to_string(), 
            RegistryEntry { 
                attributes: Rc::new(as_dyn_parser::<fcntl::FileControlAttrs>),
                trackers: None,   
        }),
        ("pread64".to_string(), 
            RegistryEntry { 
                attributes: Rc::new(as_dyn_parser::<read_write::IORequestAttrs>),
                trackers: Some(Box::new(as_dyn_tracker::<read_write::IORequestTracker>))  
        }),
        ("pwrite64".to_string(), 
            RegistryEntry { 
                attributes: Rc::new(as_dyn_parser::<read_write::IORequestAttrs>),
                trackers: Some(Box::new(as_dyn_tracker::<read_write::IORequestTracker>))  
        }),
        ("write".to_string(), 
            RegistryEntry { 
                attributes: Rc::new(as_dyn_parser::<read_write::IORequestAttrs>),
                trackers: Some(Box::new(as_dyn_tracker::<read_write::IORequestTracker>)),   
        }),
        ("read".to_string(), 
            RegistryEntry { 
                attributes: Rc::new(as_dyn_parser::<read_write::IORequestAttrs>),
                trackers: Some(Box::new(as_dyn_tracker::<read_write::IORequestTracker>)),   
        }),
        ("sendto".to_string(), 
            RegistryEntry { 
                attributes: Rc::new(as_dyn_parser::<recv_send::SocketTransferAttrs>),
                trackers: None,   
        }),
        ("recvfrom".to_string(), 
            RegistryEntry { 
                attributes: Rc::new(as_dyn_parser::<recv_send::SocketTransferAttrs>),
                trackers: None,   
        }),
        ("clone".to_string(), 
            RegistryEntry { 
                attributes: Rc::new(as_dyn_parser::<clone::CloneAttrs>),
                trackers: None,   
        }),
        ("close".to_string(), 
            RegistryEntry { 
                attributes: Rc::new(as_dyn_parser::<close::CloseFileAttrs>),
                trackers: Some(Box::new(as_dyn_tracker::<close::FileDescriptorTracker>))   
        }),
        ("close_range".to_string(), 
            RegistryEntry { 
                attributes: Rc::new(as_dyn_parser::<close::CloseRangeAttrs>),
                trackers: Some(Box::new(as_dyn_tracker::<close::CloseRangeTracker>))
        }),
        ("sendmsg".to_string(), 
            RegistryEntry { 
                attributes: Rc::new(as_dyn_parser::<messages::SocketMessageAttrs>),
                trackers: None,   
        }),
        ("recvmsg".to_string(), 
            RegistryEntry { 
                attributes: Rc::new(as_dyn_parser::<messages::SocketMessageAttrs>),
                trackers: None,   
        }),
        ("epoll_create".to_string(), 
            RegistryEntry { 
                attributes: Rc::new(as_dyn_parser::<epoll_create::EpollCreationAttrs>),
                trackers: None,   
        }),
        ("epoll_create1".to_string(), 
            RegistryEntry { 
                attributes: Rc::new(as_dyn_parser::<epoll_create::EpollCreate1Attrs>),
                trackers: None,
        }),
        ("epoll_ctl".to_string(), 
            RegistryEntry { 
                attributes: Rc::new(as_dyn_parser::<epoll_ctl::EpollControlAttrs>),
                trackers: None,   
        }),
        ("epoll_wait".to_string(), 
            RegistryEntry { 
                attributes: Rc::new(as_dyn_parser::<epoll_wait::EpollWaitAttrs>),
                trackers: None,   
        }),
        ("epoll_pwait2".to_string(), 
            RegistryEntry { 
                attributes: Rc::new(as_dyn_parser::<epoll_wait::EpollPwait2Args>),
                trackers: None,
        }),
        ("eventfd2".to_string(), 
            RegistryEntry { 
                attributes: Rc::new(as_dyn_parser::<eventfd::Eventfd2Attrs>),
                trackers: None,   
        })
    ]);
}
