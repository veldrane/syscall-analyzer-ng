use std::{arch::x86_64::_CMP_NEQ_OQ, result::Result, u8};

use regex::Regex;

use crate::general::{Parser, General, Syscall, SyscallKey, ArgumentsError};


const ARGUMENTS: &str = "(?P<dirfd>.*), \"(?P<object>.*)\", (?P<mode>.*)";
const RESULTS: &str = r"(?P<fd>\d+)<(?P<result_object>.*)> <(?P<duration>\d+\.\d+)>";

pub struct Open {
    dirfd: String,
    object: String,
    mode: String,
    fd: i32,
    result_object: String,
}


impl Open {
    pub fn new(general: &General) -> Option<Open> {

        let mut syscall: Open = Open {
            dirfd: "".to_string(),
            object: "".to_string(),
            mode: "".to_string(),
            fd: 0,
            result_object: "".to_string(),
        };

        let re_arguments = Regex::new(ARGUMENTS).unwrap();
       //let re_results = Regex::new(RESULTS).unwrap();

        match re_arguments.captures(&general.arguments) {
            Some(args) => {
                syscall.dirfd = args["dirfd"].to_string();
                syscall.object = args["object"].to_string();
                syscall.mode = args["mode"].to_string();
            },
            None => {
                println!("None openat arguments found {}", re_arguments);
                return None
            }
        };

        //match re_results.captures(&general.results) {
        //    Some(results) => {
        //       syscall.result_object = results["result_object"].to_string();
        //    },
            
        //    None => {
        //        println!("None openat results: {}", re_results);
        //        return None
         //   }
        //};

        match general.results.split_once('<') {
            Some(f) => {
                match  f.0.parse::<i32>(){
                    Ok(fd) => syscall.fd = fd,
                    _ => (),
                };
            },
            _ => (),
        };


        return Some(syscall);

    }
}

impl Parser for Open {
    fn get_arguments(&self, syscall: &mut Syscall) -> Result<(), ArgumentsError> {
        syscall.insert("dirfd".to_string(), SyscallKey::Str(self.dirfd.clone()));
        syscall.insert("object".to_string(), SyscallKey::Str(self.object.clone()));
        syscall.insert("mode".to_string(), SyscallKey::Str(self.mode.clone()));
        syscall.insert("fd".to_string(), SyscallKey::Int(self.fd));
        syscall.insert("result_object".to_string(), SyscallKey::Str(self.result_object.clone()));
        Result::Ok(())
    }
}

fn hex_to_string(input: String) -> String {
    let re = Regex::new(r"\\x([0-9a-fA-F]{2})").unwrap();

    let result = re.replace_all(&input, |caps: &regex::Captures| {
        let hex_value = &caps[1];
        let char_value = u8::from_str_radix(hex_value, 16).unwrap() as char;
        char_value.to_string()
    });

    result.to_string()
}