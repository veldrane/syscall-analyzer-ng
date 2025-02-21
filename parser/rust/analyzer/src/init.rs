use crate::registry::{SyscallArguments, Registry};
use crate::mmap;
use crate::openat;
use crate::socket;
use crate::accept;
use crate::listen;


pub fn init_registry() -> Registry {

    let mut registry = Registry::new();

    mmap::MmapArguments::register(&mut registry, "mmap");
    openat::OpenatArguments::register(&mut registry, "openat");
    socket::SocketArgs::register(&mut registry, "socket");
    accept::AcceptArgs::register(&mut registry, "accept");
    accept::AcceptArgs::register(&mut registry, "accept4");
    accept::AcceptArgs::register(&mut registry, "connect");
    accept::AcceptArgs::register(&mut registry, "bind");
    listen::ListenArgs::register(&mut registry, "listen");


    return registry;
    //OpenArguments::register(&mut registry, "open");

}