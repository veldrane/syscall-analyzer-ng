use std::result::Result;

use regex::Regex;

use crate::general::{Argumetable, General, Syscall, SyscallKey, ArgumentsError};

pub struct Network {
    fd: i32,
    socket: String,
    filename: String,
}
