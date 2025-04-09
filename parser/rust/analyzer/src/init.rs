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
                trackers: Some(Box::new(as_dyn_tracker::<mmap::MMapFilesTracker>))      
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
        }),
        ("execve".to_string(), 
        RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<execve::ExecveAttrs>),
            trackers: None,
        }),
        ("waitpid".to_string(), 
        RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<waitpid::WaitpidAttrs>),
            trackers: None,
        }),
        ("fadvise64".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<fadvise64::Fadvise64Attrs>),
            trackers: None,
        }),
        ("fallocate".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<fallocate::FallocateAttrs>),
            trackers: None,
        }),
        ("fdatasync".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<fdatasync::FdatasyncAttrs>),
            trackers: None,
        }),
        ("fgetxattr".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<fgetxattr::FgetxattrAttrs>),
            trackers: None,
        }),
        ("flistxattr".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<flistxattr::FlistxattrAttrs>),
            trackers: None,
        }),
        ("flock".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<flock::FlockAttrs>),
            trackers: None,
        }),
        ("fork".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<fork::ForkAttrs>),
            trackers: None,
        }),
        ("fsync".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<fsync::FsyncAttrs>),
            trackers: None,
        }),
        ("ftruncate".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<ftruncate::FtruncateAttrs>),
            trackers: None,
        }),
        ("futex".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<futex::FutexAttrs>),
            trackers: None,
        }),
        ("futex_wait".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<futex_wait::FutexWaitAttrs>),
            trackers: None,
        }),
        ("futex_requeue".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<futex_requeue::FutexRequeueAttrs>),
            trackers: None,
        }),
        ("futex_wake".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<futex_wake::FutexWakeAttrs>),
            trackers: None,
        }),
        ("getpeername".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<getpeername::GetpeernameAttrs>),
            trackers: None,
        }),
        ("getrlimit".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<getrlimit::GetrlimitAttrs>),
            trackers: None,
        }),
        ("inotify_init".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<inotify_init::InotifyInitAttrs>),
            trackers: None,
        }),
        ("ioctl".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<ioctl::IoctlAttrs>),
            trackers: None,
        }),
        ("madvise".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<madvise::MadviseAttrs>),
            trackers: None,
        }),
        ("membarrier".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<membarrier::MembarrierAttrs>),
            trackers: None,
        }),
        ("memfd_create".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<memfd_create::MemfdCreateAttrs>),
            trackers: None,
        }),
        ("memfd_secret".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<memfd_secret::MemfdSecretAttrs>),
            trackers: None,
        }),
        ("mlock2".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<mlock2::Mlock2Attrs>),
            trackers: None,
        }),
        ("poll".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<poll::PollAttrs>),
            trackers: None,
        }),
        ("ppoll".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<ppoll::PpollAttrs>),
            trackers: None,
        }),
        ("prctl".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<prctl::PrctlAttrs>),
            trackers: None,
        }),
        ("process_madvise".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<process_madvise::ProcessMadviseAttrs>),
            trackers: None,
        }),
        ("rt_sigaction".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<rt_sigaction::RtSigactionAttrs>),
            trackers: None,
        }),
        ("seccomp".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<seccomp::SeccompAttrs>),
            trackers: None,
        }),
        ("semctl".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<semctl::SemctlAttrs>),
            trackers: None,
        }),
        ("setns".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<setns::SetnsAttrs>),
            trackers: None,
        }),
        ("sysinfo".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<sysinfo::SysinfoAttrs>),
            trackers: None,
        }),
        ("timerfd_create".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<timerfd_create::TimerfdCreateAttrs>),
            trackers: None,
        }),
        ("waitid".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<waitid::WaitidAttrs>),
            trackers: None,
        }),
        ("arch_prctl".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<arch_prctl::ArchPrctlAttrs>),
            trackers: None,
        }),
        ("brk".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<brk::BrkAttrs>),
            trackers: None,
        }),
        ("chdir".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<chdir::ChdirAttrs>),
            trackers: None,
        }),
        ("fstat".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<fstat::FstatAttrs>),
            trackers: None,
        }),
        ("fstatfs".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<fstatfs::FstatfsAttrs>),
            trackers: None,
        }),
        ("getdents64".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<getdents64::Getdents64Attrs>),
            trackers: None,
        }),
        ("getrandom".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<getrandom::GetrandomAttrs>),
            trackers: None,
        }),
        ("getsockname".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<getsockname::GetsocknameAttrs>),
            trackers: None,
        }),
        ("lseek".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<lseek::LseekAttrs>),
            trackers: None,
        }),
        ("newfstatat".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<newfstatat::NewfstatatAttrs>),
            trackers: None,
        }),
        ("pipe2".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<pipe2::Pipe2Attrs>),
            trackers: None,
        }),
        ("prlimit64".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<prlimit64::Prlimit64Attrs>),
            trackers: None,
        }),
        ("rseq".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<rseq::RseqAttrs>),
            trackers: None,
        }),
        ("rt_sigprocmask".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<rt_sigprocmask::RtSigprocmaskAttrs>),
            trackers: None,
        }),
        ("setgroups".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<setgroups::SetgroupsAttrs>),
            trackers: None,
        }),
        ("set_robust_list".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<set_robust_list::SetRobustListAttrs>),
            trackers: None,
        }),
        ("set_tid_address".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<set_tid_address::SetTidAddressAttrs>),
            trackers: None,
        }),
        ("statfs".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<statfs::StatfsAttrs>),
            trackers: None,
        }),
        ("umask".to_string(), RegistryEntry { 
            attributes: Rc::new(as_dyn_parser::<umask::UmaskAttrs>),
            trackers: None,
        }),
    ]);
}
