use crate::registry::{SyscallArguments, Registry};
use crate::{accept, clone, close, epoll_create, fcntl, listen, mmap, munmap, openat, pread64, sendmsg, sendto, socket};


pub fn init_registry() -> Registry {

    let mut registry = Registry::new();

    mmap::MmapArguments::register(&mut registry, "mmap");
    munmap::MunmapArguments::register(&mut registry, "munmap");
    openat::OpenatArguments::register(&mut registry, "openat");
    socket::SocketArgs::register(&mut registry, "socket");
    accept::AcceptArgs::register(&mut registry, "accept");
    accept::AcceptArgs::register(&mut registry, "accept4");
    accept::AcceptArgs::register(&mut registry, "connect");
    accept::AcceptArgs::register(&mut registry, "bind");
    listen::ListenArgs::register(&mut registry, "listen");
    fcntl::FcntlArgs::register(&mut registry, "fcntl");
    pread64::ReadArgs::register(&mut registry, "read");
    pread64::ReadArgs::register(&mut registry, "pread64");
    pread64::ReadArgs::register(&mut registry, "write");
    pread64::ReadArgs::register(&mut registry, "pwrite64");
    sendto::SendtoArgs::register(&mut registry, "sendto");
    sendto::SendtoArgs::register(&mut registry, "recvfrom");
    clone::CloneArgs::register(&mut registry, "clone");
    close::CloseArgs::register(&mut registry, "close");
    sendmsg::SendmsgArgs::register(&mut registry, "sendmsg");
    sendmsg::SendmsgArgs::register(&mut registry, "recvmsg");
    epoll_create::EpollCreateArgs::register(&mut registry, "epoll_create");
    epoll_create::EpollCreate1Args::register(&mut registry, "epoll_create1");
    
    return registry;
    //OpenArguments::register(&mut registry, "open");

}