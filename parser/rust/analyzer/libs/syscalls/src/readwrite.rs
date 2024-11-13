use std::result::Result;

use regex::Regex;

use crate::general::{Argumetable, General, Syscall, SyscallKey, ArgumentsError};

// 	"read" :        '(?P<fd>\d+)\<(?P<objectname>.*)\>\,\s(?P<data>.*)\,\s(?P<size>\d+)',
//"pread64" :     '(?P<fd>\d+)\<(?P<objectname>.*)\>\,\s(?P<data>.*)\,\s(?P<size>\d+),\s(?P<offset>\d+)',
//    #	"write" :       '(?P<fd>\d+).*\,\s(?P<data>.*)\,\s(?P<size>\d+)',
//        "write" :        '(?P<fd>\d+)\<(?P<objectname>.*)\>\,\s(?P<data>.*)\,\s(?P<size>\d+)',
 //       "pwrite64" :     '(?P<fd>\d+)\<(?P<objectname>.*)\>\,\s(?P<data>.*)\,\s(?P<size>\d+),\s(?P<offset>\d+)',

 const RESULTS: &str = r"(?P<result_size>\d+) <(?P<duration>\d+\.\d+)>";
 pub struct ReadWrite {
    fd: i32,
    filename: String,
    data: String,
    size: i32,
    offset: String,
    result_size: i32
}


impl ReadWrite {

    pub fn new(general: &General) -> Option<ReadWrite> {

        let re_results = Regex::new(RESULTS).unwrap();

        let mut syscall  = ReadWrite{
            fd: 0,
            filename: "".to_string(),
            data: "".to_string(),
            size: 0,
            offset: "".to_string(),
            result_size: 0
        };

        let arguments: Vec<String> = general.arguments.split(',')
        .map(|s| s.trim().to_string())
        .collect();


        if arguments.len()  < 3 {
            return None
        }

        if arguments[0].contains('<') {
            let fd_filename: Vec<String> = arguments[4].split('<')
                                  .map(|s| s.trim().to_string())
                                  .collect();
            
            syscall.fd = fd_filename[0].parse().unwrap();
            syscall.filename = fd_filename[1].replace(">", "");

        } else {
            syscall.fd = arguments[0].parse().unwrap();
        }

        syscall.data = arguments[1].to_string();
        syscall.size = arguments[2].parse().unwrap();
        syscall.offset = arguments[3].to_string();

        match re_results.captures(&general.results) {
            Some(results) => {
                syscall.result_size = results["result_size"].parse().unwrap();
            },
            None => return None
        };

        Some(syscall)

    }
}

impl Argumetable for ReadWrite {
    fn get_arguments(&self, syscall: &mut Syscall) -> Result<(), ArgumentsError> {
        syscall.insert("fd".to_string(), SyscallKey::Int(self.fd));
        syscall.insert("filename".to_string(), SyscallKey::Str(self.filename.clone()));
        syscall.insert("data".to_string(), SyscallKey::Str(self.data.clone()));
        syscall.insert("size".to_string(), SyscallKey::Int(self.size));
        syscall.insert("offset".to_string(), SyscallKey::Str(self.offset.clone()));
        syscall.insert("result_size".to_string(), SyscallKey::Int(self.result_size.clone()));
        Result::Ok(())
    }
}