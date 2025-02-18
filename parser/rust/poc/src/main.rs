use std::collections::HashMap;


use serde_json;
use std::fs::read_to_string;
use syscalls::general::{General, Parser, Syscall, SyscallKey};
use syscalls::mmap::*;
use syscalls::network::*;
use syscalls::open::*;
use syscalls::readwrite::*;

pub mod examples;

const strace_output: &str = "../../../tests/curl/curl.108960";

fn main() {
    // let general = General::new(&BIND).unwrap();

    let mut syscalls: Vec<HashMap<String, SyscallKey>> = vec![];
    let mut syscall: Syscall = Syscall(HashMap::new());

    for line in read_to_string(strace_output).unwrap().lines() {

        println!("{}", line);
        let general = match General::new(&line) {
            Some(g) => g,
            None => continue,
        };

        general.get_arguments(&mut syscall).ok();

        match general.syscall.as_str() {
            "openat" => {
                Open::new(&general)
                    .unwrap()
                    .get_arguments(&mut syscall)
                    .ok();
            }
            "mmap" => {
                Mmap::new(&general)
                    .unwrap()
                    .get_arguments(&mut syscall)
                    .ok();
            }
            "read" | "read64 " | "write" | "write64" => {
                ReadWrite::new(&general)
                    .unwrap()
                    .get_arguments(&mut syscall)
                    .ok();
            }
            "accept" | "bind" | "connect" => {
                Network::new(&general)
                    .unwrap()
                    .get_arguments(&mut syscall)
                    .ok();
            }
            _ => {
                println!("No match");
            }
        };

        syscalls.push(syscall.0.clone());
    }

    let json = serde_json::to_string(&syscalls).unwrap();
    println!("{}", json);
}
