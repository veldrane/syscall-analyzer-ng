use crate::registry::{SyscallArguments, Registry};
use crate::mmap;
use crate::open;


pub fn init_registry() -> Registry {

    let mut registry = Registry::new();

    mmap::MmapArguments::register(&mut registry, "mmap");
    open::OpenArguments::register(&mut registry, "open");
    open::OpenArguments::register(&mut registry, "openat");
    return registry;
    //OpenArguments::register(&mut registry, "open");

}