use serde::{Deserialize, Serialize};
use wrappers::parsers::Parsable;
use wrappers::trackers::Trackable;
use std::str::FromStr;
use helpers::helpers::{split_fd_parts_to_strings, split_fd_parts, HexString};
use std::any::Any;
use std::rc::Rc;
use trackers::descriptors::{Descs, DescType};


#[derive(Debug,Serialize,Deserialize, Default)]
pub struct OpenatArguments {
    dirfd: String,
    path: String,
    requested_file_name: String,
    flags: String,
    mode: String,
    fd: i32,
    file_name: String,
}


#[derive(Debug,Serialize,Deserialize, Default)]
pub struct OpenatTrack {
    uuid: String,
}

#[typetag::serde]
impl Parsable for OpenatArguments {
    fn parse(args: &str, result: Option<&str>) -> Result<Self, String> {

        let mut openat_syscall = OpenatArguments::default();

        let parts: Vec<String> = args
                                    .chars()
                                    .filter(|&c| !r#""\"? "#.contains(c))
                                    .collect::<String>()
                                    .split(',')
                                    .map(str::to_string)
                                    .collect::<Vec<String>>();
        if parts.len() < 3 {
            return Err("Invalid number of arguments".into());
        }

        let (dirfd, object) = split_fd_parts_to_strings(&parts[0]);

        if parts.len() == 4 {
            openat_syscall.mode = parts[3].to_string();
        }

        openat_syscall.dirfd = dirfd;
        openat_syscall.path = object;
        openat_syscall.requested_file_name = HexString::from_str(&parts[1]).unwrap().to_string();
        openat_syscall.flags = parts[2].to_string();

        (openat_syscall.fd, openat_syscall.file_name) = match result {
            Some(r) => split_fd_parts(&r),
            None => (0, "".to_string())
        };

        Ok(openat_syscall)
    }
    
    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }

}

#[typetag::serde]
impl Trackable for OpenatTrack {
    fn track(descs: &mut Descs, timestamp: f64, attrs: Rc<dyn Parsable>) -> Result<Self, String> {

        // Pokusíme se downcastnout na Box<SocketArgs>

        // eprint!("Socket track: \n");

        let openat_args: Rc<OpenatArguments> = attrs
            .as_any()
            .downcast::<OpenatArguments>()
            .map_err(|_| "failed downcast to SocketArgs".to_string())?;


        if openat_args.fd == -1 {
            return Err("Socket fd is 0".to_string());
        }
        

        let uuid = match descs.add(
            timestamp,
            openat_args.fd,
            openat_args.file_name.clone(),
            DescType::File,
        ) {
            Ok(uuid) => uuid,
            Err(_) => {
            //    eprintln!("Error adding socket descriptor");
                return Err("No uuid found".to_string()) 
            }
        };

        // eprintln!("Socket track uuid: {}", uuid);
        
        Ok(OpenatTrack {
            uuid: uuid
        })
    }
}