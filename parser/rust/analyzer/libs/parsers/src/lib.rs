pub mod syscall;
pub mod default;
pub mod mmap;
pub mod openat;
pub mod socket;
pub mod network;
pub mod listen;
pub mod fcntl;
pub mod read_write;
pub mod recv_send;
pub mod clone;
pub mod close;
pub mod munmap;
pub mod messages;
pub mod epoll_create;
pub mod epoll_ctl;
pub mod epoll_wait;
pub mod dup2;
pub mod access;
pub mod mprotect;
pub mod eventfd;
pub mod execve;
pub mod getpid;
pub mod waitpid;
pub mod fadvise64;
pub mod fallocate;
pub mod fdatasync;
pub mod fgetxattr;
pub mod flistxattr;
pub mod flock;
pub mod fork;
pub mod fsync;
pub mod ftruncate;
pub mod futex;
pub mod futex_wait;
pub mod futex_requeue;
pub mod futex_wake;
pub mod getpeername;
pub mod getrlimit;
pub mod inotify_init;
pub mod ioctl;
pub mod madvise;
pub mod memfd_create;
pub mod memfd_secret;
pub mod membarrier;
pub mod mlock2;
pub mod poll;
pub mod ppoll;
pub mod prctl;
pub mod process_madvise;
pub mod rt_sigaction;
pub mod seccomp;
pub mod semctl;
pub mod setns;
pub mod sysinfo;
pub mod timerfd_create;
pub mod waitid;
pub mod arch_prctl;
pub mod brk;
pub mod chdir;
pub mod fstat;
pub mod fstatfs;
pub mod getdents64;
pub mod getrandom;
pub mod getsockname;
pub mod lseek;
pub mod newfstatat;
pub mod pipe2;
pub mod prlimit64;
pub mod rseq;
pub mod rt_sigprocmask;
pub mod setgroups;
pub mod set_robust_list;
pub mod set_tid_address;
pub mod statfs;
pub mod umask;
