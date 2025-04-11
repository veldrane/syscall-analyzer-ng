use helpers::helpers::split_fd_parts;
use wrappers::parsers::Parsable;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::rc::Rc;
use trackers::fd_table::Descs;
use wrappers::trackers::Trackable;

#[derive(Debug, Serialize,Deserialize)]
pub struct CloseFileAttrs {
    fd: i32,
    file_name: String,
}


#[derive(Debug, Serialize,Deserialize)]
pub struct CloseRangeAttrs {
    min_fd: i32,
    max_fd: i32,
    mask: i32,
}

#[derive(Debug, Serialize,Deserialize)]
pub struct FileDescriptorTracker {
    uuid: String,
    parrent: Option<String>,
}


#[derive(Debug, Serialize,Deserialize)]
pub struct CloseRangeTracker {}

#[typetag::serde]
impl Parsable for CloseFileAttrs {
    fn parse(args: &str, _: Option<&str>) -> Result<Self, String> {

        let (fd, file_name ) = split_fd_parts(&args);

        Ok(CloseFileAttrs {
            fd: fd,
            file_name: file_name,
        })
    }

    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }  
}

#[typetag::serde]
impl Trackable for FileDescriptorTracker {
    fn track(descs: &mut Descs, timestamp: f64, attrs: Rc<dyn Parsable>) -> Result<Self, String> {

        // Pokusíme se downcastnout na Box<SocketArgs>

        //eprint!("Socket track: \n");

        let args: Rc<CloseFileAttrs> = attrs
            .as_any()
            .downcast::<CloseFileAttrs>()
            .map_err(|_| "failed downcast to ReadWriteArgs".to_string())?;


        if args.fd == -1 {
            return Err("Socket fd is -1".to_string());
        }
        

        let record = match descs.get_fd(args.fd) {
            Some(record) => record,
            None => {
                return Err("No uuid found".to_string()) 
            }
        };

        let uuid = record.uuid.clone();
        let parrent = record.parrent .clone();

        //eprintln!("Close track uuid: {}", uuid);
        descs.close(&uuid, timestamp).map_err(|_| {
            eprintln!("Error closing descriptor");
            "Error closing descriptor".to_string()
        })?;


        // eprintln!("Socket track uuid: {}", uuid);
        
        Ok(FileDescriptorTracker {
            uuid: uuid,
            parrent: parrent
        })
    }
}


#[typetag::serde]
impl Parsable for CloseRangeAttrs {
    fn parse(args: &str, _: Option<&str>) -> Result<Self, String> {

        let parts: Vec<String> = args
                                    .chars()
                                    .filter(|&c| !r#""\"? "#.contains(c))
                                    .collect::<String>()
                                    .split(',')
                                    .map(str::to_string)
                                    .collect::<Vec<String>>();

        Ok(CloseRangeAttrs {
            min_fd: parts[0].parse::<i32>().unwrap_or(0),
            max_fd: parts[1].parse::<i32>().unwrap_or(0),
            mask: parts[2].parse::<i32>().unwrap_or(0),
        })
    }

    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }  
}

#[typetag::serde]
impl Trackable for CloseRangeTracker {
    fn track(descs: &mut Descs, timestamp: f64, attrs: Rc<dyn Parsable>) -> Result<Self, String> {


        let args: Rc<CloseRangeAttrs> = attrs
            .as_any()
            .downcast::<CloseRangeAttrs>()
            .map_err(|_| "failed downcast to ReadWriteArgs".to_string())?;

            
        match descs.close_range(args.min_fd, args.max_fd, timestamp) {
            Ok(_) => {}
            Err(_) => {
                return Err("Error closing descriptor".to_string())
            }
        };

        Ok(CloseRangeTracker {})
    }
    
}