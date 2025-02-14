use std::result::Result;

use regex::Regex;


use crate::general::{ArgumentsError, General, Parser, Syscall, SyscallKey};

#[derive(Clone,Debug)]
pub struct AfInet {
    in_port: i32,
    in_address: String,
}

#[derive(Clone,Debug)]
pub struct AfUnix {
    path: String,
}

#[derive(Clone, Debug)]
enum SocketAddress {
    AfInet(AfInet),
    AfUnix(AfUnix),
    None
}

#[derive(Debug)]
pub struct Network {
    fd: i32,
    socket: String,
    socket_address: Option<SocketAddress>,
    socket_family: String,
    result_code: i32,
    result_fd: i32,
    result_socket: String
}

impl Network {
    pub fn new(general: &General) -> Option<Network> {
        
        let mut syscall: Network = Network {
            fd: 0,
            socket: "".to_string(),
            socket_address: None,
            socket_family: "".to_string(),
            result_code: 0,
            result_fd: 0,
            result_socket: "".to_string(),
        };

        let arguments: Vec<String> = general.arguments.split(',')
                                  .map(|s| s.trim().to_string())
                                  .collect();

        if arguments[0].contains('<') {
            let fd_filename: Vec<String> = arguments[0].split('<')
                                                        .map(|s| s.trim().to_string())
                                                        .collect();

            syscall.fd = fd_filename[0].parse().unwrap();
            syscall.socket = fd_filename[1].replace(">", "");
        } else {
            syscall.fd = arguments[0].parse().unwrap();
        }

        if arguments[1].contains('=') {
            let socket_family: Vec<String> = arguments[1].split('=')
                                                        .map(|s| s.trim().to_string())
                                                        .collect();
            
            syscall.socket_family = socket_family[1].to_string();

        }

        let socket_address = match syscall.socket_family.as_str() {
            "AF_INET" => SocketAddress::AfInet(AfInet::new(general).unwrap()),
            "AF_UNIX" => SocketAddress::AfUnix(AfUnix::new(general).unwrap()),
            _ => SocketAddress::None
        };

        syscall.socket_address = Some(socket_address.clone());

        let results = general.results.split(' ')
                                  .map(|s| s.trim().to_string())
                                  .collect::<Vec<String>>();

        if results[0].contains("<") {
            let result: Vec<String> = results[0].split('<')
                                  .map(|s| s.trim().to_string())
                                  .collect();
            
            syscall.result_fd = result[0].parse().unwrap();
            syscall.result_socket = result[1].replace(">", "");
        } else {
            syscall.result_code = results[0].parse().unwrap();
        }

        Some(syscall)
    }
}


impl AfInet {

    fn new(general: &General) -> Option<AfInet> {

        let re_afinet = Regex::new(r#"sin_port=htons\((\d+)\), sin_addr=inet_addr\(\"([0-9]+\.[0-9]+\.[0-9]+\.[0-9]+)\"\)"#).unwrap();

        match re_afinet.captures(&general.arguments) {
            Some(args) => {
                Some(AfInet {
                    in_port: args[1].parse().unwrap(),
                    in_address: args[2].to_string(),
                })
            },
            None => None
        }
    }
}


impl AfUnix {

    fn new(general: &General) -> Option<AfUnix> {

        let re_afinet = Regex::new(r#"sun_path=\"([^\"]+)\""#).unwrap();
        match re_afinet.captures(&general.arguments) {
            Some(args) => {
                Some(AfUnix {
                    path: args[1].parse().unwrap(),
                })
            },
            None => {
                None
            }
        }
    }
}

impl Parser for Network {

    fn get_arguments(&self, syscall: &mut Syscall) -> Result<(), ArgumentsError> {
        match &self.socket_address {
            Some(SocketAddress::AfInet(af_inet)) => {
                syscall.insert("in_port".to_string(), SyscallKey::Str((af_inet.in_port).to_string()));
                syscall.insert("in_address".to_string(), SyscallKey::Str(af_inet.in_address.clone()));
            },
            Some(SocketAddress::AfUnix(af_unix)) => {
                syscall.insert("path".to_string(), SyscallKey::Str(af_unix.path.clone()));
            },
            _ => println!("Socket address not found.")
        };

        syscall.insert("fd".to_string(), SyscallKey::Int(self.fd));
        syscall.insert("socket".to_string(), SyscallKey::Str(self.socket.clone()));
        syscall.insert("socket_family".to_string(), SyscallKey::Str(self.socket_family.clone()));

        if self.result_socket != "" {
            syscall.insert("result_socket".to_string(), SyscallKey::Str(self.result_socket.clone()));
            syscall.insert("result_fd".to_string(), SyscallKey::Int(self.result_fd));
        } else {
            syscall.insert("result_code".to_string(), SyscallKey::Int(self.result_code));
        }

        Result::Ok(())
    }
}