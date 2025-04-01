use helpers::helpers::split_fd_parts;
use wrappers::parsers::Parsable;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::rc::Rc;
use trackers::descriptors::Descs;
use wrappers::trackers::Trackable;

#[derive(Debug, Serialize,Deserialize)]
pub struct CloseArgs {
    fd: i32,
    file_name: String,
}


#[derive(Debug, Serialize,Deserialize)]
pub struct CloseTrack {
    uuid: String,
}


#[typetag::serde]
impl Parsable for CloseArgs {
    fn parse(args: &str, _: Option<&str>) -> Result<Self, String> {

        let (fd, file_name ) = split_fd_parts(&args);

        Ok(CloseArgs {
            fd: fd,
            file_name: file_name,
        })
    }

    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }  
}

#[typetag::serde]
impl Trackable for CloseTrack {
    fn track(descs: &mut Descs, timestamp: f64, attrs: Rc<dyn Parsable>) -> Result<Self, String> {

        // Pokusíme se downcastnout na Box<SocketArgs>

        //eprint!("Socket track: \n");

        let args: Rc<CloseArgs> = attrs
            .as_any()
            .downcast::<CloseArgs>()
            .map_err(|_| "failed downcast to ReadWriteArgs".to_string())?;


        if args.fd == -1 {
            return Err("Socket fd is -1".to_string());
        }
        

        let uuid = match descs.get_by_descriptor_number(args.fd) {
            Some(record) => record.uuid.clone(),
            None => {
                return Err("No uuid found".to_string()) 
            }
        };

        //eprintln!("Close track uuid: {}", uuid);
        descs.close(&uuid, timestamp).map_err(|_| {
            eprintln!("Error closing descriptor");
            "Error closing descriptor".to_string()
        })?;


        // eprintln!("Socket track uuid: {}", uuid);
        
        Ok(CloseTrack {
            uuid: uuid.to_string()
        })
    }
}