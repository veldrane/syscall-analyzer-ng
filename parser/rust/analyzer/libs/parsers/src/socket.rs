use serde::{Serialize,Deserialize};
use wrappers::parsers::Parsable;
use wrappers::trackers::Trackable;
use trackers::fd_table::{Descs, DescType};
use helpers::helpers::split_fd_parts;
use std::any::Any;
use std::rc::Rc;

#[derive(Debug, Serialize,Deserialize)]
pub struct NetworkSocketAttrs {
    domain: String,
    socket_type: String,
    protocol: String,
    socket_fd: i32,
    socket_name: String,
}

#[derive(Debug, Serialize,Deserialize)]
pub struct NetworkSocketTracker {
    uuid: String,
}


// First template for tracking syscalls, maybe will be rewriten to rc<dyn trait>
#[typetag::serde]
impl Trackable for NetworkSocketTracker {
    fn track(descs: &mut Descs, timestamp: f64, attrs: Rc<dyn Parsable>) -> Result<Self, String> {

        // Pokusíme se downcastnout na Box<SocketArgs>

        // eprint!("Socket track: \n");

        let socket_args: Rc<NetworkSocketAttrs> = attrs
            .as_any()
            .downcast::<NetworkSocketAttrs>()
            .map_err(|_| "failed downcast to SocketArgs".to_string())?;


        if socket_args.socket_fd == -1 {
            return Err("Socket fd is 0".to_string());
        }
        

        let uuid = match descs.add(
            timestamp,
            socket_args.socket_fd,
            socket_args.socket_name.clone(),
            DescType::Socket,
        ) {
            Ok(uuid) => uuid,
            Err(_) => {
            //    eprintln!("Error adding socket descriptor");
                return Err("No uuid found".to_string()) 
            }
        };

        // eprintln!("Socket track uuid: {}", uuid);
        
        Ok(NetworkSocketTracker {
            uuid: uuid
        })
    }
}

#[typetag::serde]
impl Parsable for NetworkSocketAttrs {
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

        let (socket_fd, socket_name ) = match result {
            Some(r) => split_fd_parts(&r),
            None => (0, "".to_string())
        };

        Ok(NetworkSocketAttrs {
            socket_fd: socket_fd,
            socket_name: socket_name,
            domain: parts[0].to_string(),
            socket_type: parts[1].to_string(),
            protocol: parts[2].to_string()
        })
    }

    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }

}