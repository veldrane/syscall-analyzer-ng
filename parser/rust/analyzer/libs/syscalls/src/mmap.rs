use std::result::Result;

use regex::Regex;

use crate::general::{Parser, General, Syscall, SyscallKey, ArgumentsError};


//const ARGUMENTS: &str = r"(?P<addr>.*)\,\s(?P<size>\d+)\,\s(?P<protection>.*)\,\s(?P<flags>.*)\,\s*(?P<object>\d+(?:<[^>]+>)?),\s*(?P<offset>.*)";

//const ARGUMENTS: &str = r"(?P<addr>[0-9a-fA-F]+),\s*(?P<size>\d+),\s*(?P<protection>[A-Z_|]+),\s*(?P<flags>[A-Z_|]+),\s*(?P<fd>\d+)(?:<(?P<object>[^>]+)>)?,\s*(?P<offset>0x[0-9a-fA-F]+)$";

const RESULTS: &str = r"(?P<offset>.*+) <(?P<duration>\d+\.\d+)>";

pub struct Mmap {
    addr: String,
    size: i32,
    protection: String,
    flags: String,
    fd: i32,
    filename: String,
    offset: String,
    result_offset: String,
}


impl Mmap {
    pub fn new(general: &General) -> Option<Mmap> {

        let mut syscall: Mmap = Mmap {
            addr: "".to_string(),
            size: 0,
            protection: "".to_string(),
            flags: "".to_string(),
            fd: 0,
            filename: "".to_string(),
            offset: "".to_string(),
            result_offset: "".to_string(),
        };

        let re_results = Regex::new(RESULTS).unwrap();

        let arguments: Vec<String> = general.arguments.split(',')
                                  .map(|s| s.trim().to_string())
                                  .collect();
        if arguments.len() != 6 {
            return None
        }

        if arguments[4].contains('<') {
            let object: Vec<String> = arguments[4].split('<')
                                  .map(|s| s.trim().to_string())
                                  .collect();
            
            syscall.fd = object[0].parse().unwrap();
            syscall.filename = object[1].replace(">", "");

        } else {
            syscall.fd = arguments[4].parse().unwrap();
        }


        syscall.addr = arguments[0].clone();
        syscall.size = arguments[1].parse().unwrap();
        syscall.protection = arguments[2].clone();
        syscall.flags = arguments[3].clone();
        syscall.offset = arguments[5].clone();

        match re_results.captures(&general.results) {
            Some(results) => {
                syscall.result_offset = results["offset"].to_string();
            },
            None => return None
        };


        return Some(syscall);

    }
}

impl Parser for Mmap {
    fn get_arguments(&self, syscall: &mut Syscall) -> Result<(), ArgumentsError> {
        syscall.insert("addr".to_string(), SyscallKey::Str(self.addr.clone()));
        syscall.insert("size".to_string(), SyscallKey::Int(self.size));
        syscall.insert("protection".to_string(), SyscallKey::Str(self.protection.clone()));
        syscall.insert("flags".to_string(), SyscallKey::Str(self.flags.clone()));
        syscall.insert("fd".to_string(), SyscallKey::Int(self.fd));
        syscall.insert("filename".to_string(), SyscallKey::Str(self.filename.clone()));
        syscall.insert("offset".to_string(), SyscallKey::Str(self.offset.clone()));
        syscall.insert("result_offset".to_string(), SyscallKey::Str(self.result_offset.clone()));
        Result::Ok(())
    }
}
