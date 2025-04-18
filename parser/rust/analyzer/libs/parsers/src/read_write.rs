use helpers::helpers::split_fd_parts;
use wrappers::parsers::Parsable;
use wrappers::trackers::Trackable;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::rc::Rc;
use trackers::fd_table::Descs;

#[derive(Debug, Serialize,Deserialize)]
pub struct IORequestAttrs {
    fd: i32,
    file_name: String,
    buffer: String,
    requested_size: i32,
    offset: String,
    size: i32,
}

#[derive(Debug, Serialize,Deserialize)]
pub struct IORequestTracker {
    uuid: String,
}

#[typetag::serde]
impl Parsable for IORequestAttrs {
    fn parse(args: &str, result: Option<&str>) -> Result<Self, String> {

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
        
        let (fd, file_name ) = split_fd_parts(&parts[0]);

        let offset = if let Some(opt) = parts.get(3) {
            opt.to_string()
        } else {
            "".to_string()
        };

        let size = match result {
            Some(r)=>r.parse::<i32>().map_err(|e| e.to_string())?,
            None => 0
        };

        Ok(IORequestAttrs {
            fd: fd,
            file_name: file_name,
            buffer: parts[1].to_string(),
            requested_size:parts[2].parse::<i32>().unwrap(),
            offset: offset,
            size: size,
        })
    }   


    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }

}

#[typetag::serde]
impl Trackable for IORequestTracker {
    fn track(descs: &mut Descs, _: f64, attrs: Rc<dyn Parsable>) -> Result<Self, String> {

        // Pokusíme se downcastnout na Box<SocketArgs>

        // eprint!("Socket track: \n");

        //eprintln!("descriptors: {:?}\n\n\n", descs);

        let args: Rc<IORequestAttrs> = attrs
            .as_any()
            .downcast::<IORequestAttrs>()
            .map_err(|_| "failed downcast to ReadWriteArgs".to_string())?;


        if args.fd == -1 {
            return Err("Socket fd is 0".to_string());
        }
        

        let uuid = match descs.get_fd(args.fd) {
            Some(record) => &record.uuid,
            None => {
                return Err("No uuid found".to_string()) 
            }
        };

        // eprintln!("Socket track uuid: {}", uuid);
        
        Ok(IORequestTracker {
            uuid: uuid.to_string()
        })
    }
}