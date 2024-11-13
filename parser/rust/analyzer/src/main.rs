use std::collections::HashMap;

//use general::SyscallKey;

use serde_json;
use syscalls::general::{General, Syscall, SyscallKey,Parser};
use syscalls::open::*;
use syscalls::mmap::*;
use syscalls::readwrite::*;




const MMAP: &str = "1708596064.182267 mmap(0x7f0bcf06b000, 204800, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3</usr/lib/x86_64-linux-gnu/libcrypto.so.1.1>, 0x2b5000) = 0x7f0bcf06b000 <0.000017>";

//const MMAP: &str = "1730805691.281081 mmap(NULL, 12288, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7f1f1cf41000 <0.000021>";
//const OPENAT: &str = "1730805691.259985 openat(AT_FDCWD</home/veldrane>, \"/etc/ld.so.cache\", O_RDONLY|O_CLOEXEC) = 3</etc/ld.so.cache> <0.000028>>";


fn main() {

    let general = General::new(&MMAP).unwrap();
    let mut syscall: Syscall = Syscall(HashMap::new());

    general.get_arguments(&mut syscall).ok();


    match general.syscall.as_str() {
        "openat" => {
            Open::new(&general).unwrap().get_arguments(&mut syscall).ok();
            },
        "mmap" => {
            Mmap::new(&general).unwrap().get_arguments(&mut syscall).ok();
            },
        "read" | "read64 " | "write" | "write64" => {
            ReadWrite::new(&general).unwrap().get_arguments(&mut syscall).ok();
            },

        _ => {
            panic!("Unknown syscall");
        }
    };



    let mut syscalls: Vec<HashMap<String,SyscallKey>> = vec![];

    syscalls.push(syscall.0.clone());
    syscalls.push(syscall.0.clone());
    syscalls.push(syscall.0.clone());


    
    let json = serde_json::to_string(&syscalls).unwrap();
    println!("{}", json);
    

}
