use std::result::Result;

use regex::Regex;

use crate::general::{Parser, General, Syscall, SyscallKey, ArgumentsError};

pub struct AfInet {
    in_port: i32,
    in_address: String,
}

pub struct AfUnix {
    path: String,
}

enum SocketAddress {
    af_inet(AfInet),
    af_unix(AfUnix)

}

pub struct Network {
    fd: i32,
    socket: String,
    socket_address: SocketAddress,
    result_code: i32,
    result_fd: i32,
    result_socket: String
}

impl Network {
    pub fn new(general: &General) -> Option<Network> {
        unimplemented!()
    }
}
