use std::result::Result;

use regex::Regex;

use crate::general::{Argumetable, General, Syscall, SyscallKey, ArgumentsError};


const ARGUMENTS: &str = "(?P<dirfd>.*), \"(?P<object>.*)\", (?P<mode>.*)";
const RESULTS: &str = r"(?P<fd>\d+)<(?P<result_object>.*)> <(?P<duration>\d+\.\d+)>";

pub struct Openat {
    dirfd: String,
    object: String,
    mode: String,
    fd: i32,
    result_object: String,
}


impl Openat {
    pub fn new(general: &General) -> Option<Openat> {

        let mut syscall: Openat = Openat {
            dirfd: "".to_string(),
            object: "".to_string(),
            mode: "".to_string(),
            fd: 0,
            result_object: "".to_string(),
        };

        let re_arguments = Regex::new(ARGUMENTS).unwrap();
        let re_results = Regex::new(RESULTS).unwrap();

        match re_arguments.captures(&general.arguments) {
            Some(args) => {
                syscall.dirfd = args["dirfd"].to_string();
                syscall.object = args["object"].to_string();
                syscall.mode = args["mode"].to_string();
            },
            None => {
                println!("None mmap results");
                return None
            }
        };

        match re_results.captures(&general.results) {
            Some(results) => {
                syscall.result_object = results["result_object"].to_string();
            },
            
            None => {
                println!("None mmap results");
                return None
            }
        };


        return Some(syscall);

    }
}

impl Argumetable for Openat {
    fn get_arguments(&self, syscall: &mut Syscall) -> Result<(), ArgumentsError> {
        syscall.insert("dirfd".to_string(), SyscallKey::Str(self.dirfd.clone()));
        syscall.insert("object".to_string(), SyscallKey::Str(self.object.clone()));
        syscall.insert("mode".to_string(), SyscallKey::Str(self.mode.clone()));
        syscall.insert("fd".to_string(), SyscallKey::Int(self.fd));
        syscall.insert("result_object".to_string(), SyscallKey::Str(self.result_object.clone()));
        Result::Ok(())
    }
}
