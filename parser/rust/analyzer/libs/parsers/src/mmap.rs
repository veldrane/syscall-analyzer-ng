use serde::{Deserialize, Serialize};
use helpers::helpers::split_fd_parts;
use helpers::converts::hex_serde_u64;
use helpers::converts::hex_serde_u16;
use wrappers::parsers::Parsable;
use std::any::Any;
use std::rc::Rc;
use wrappers::trackers::Trackable;
use trackers::fd_table::Descs;

#[derive(Debug, Serialize,Deserialize)]
pub struct MmapAttrs {
    #[serde(with = "hex_serde_u64")]
    requested_addr: u64,
    size: i32,
    protection: String,
    flags: String,
    fd: i32,
    file_name: String,
    #[serde(with = "hex_serde_u16")]
    offset: u16,
    #[serde(with = "hex_serde_u64")]
    addr:u64,
}


#[typetag::serde]
impl Parsable for MmapAttrs {
    fn parse(args: &str, result: Option<&str>) -> Result<Self, String>{

        let mut file_name = "".to_string();
        let mut fd = -1;

        let parts: Vec<String> = args
                                    .chars()
                                    .filter(|&c| !r#""\"? "#.contains(c))
                                    .collect::<String>()
                                    .split(',')
                                    .map(str::to_string)
                                    .collect::<Vec<String>>();

        if parts.len() != 6 {
            return Err("Invalid number of arguments".into());
        }

        if !parts[4].contains("-1") {
            (fd, file_name) = split_fd_parts(&parts[4]);
        }

        let addr = match result {
            Some(r) => u64::from_str_radix(&r[2..], 16).unwrap_or(0),
            None => 0,
        };

        let offset = if parts[5].len() > 2 { 
                match u16::from_str_radix(&parts[5][2..], 16) {
                Ok(o) => o,
                Err(_) => 0,
            }
        } else {
            0
        };

        Ok(MmapAttrs {
            requested_addr: u64::from_str_radix(&parts[0][2..], 16).unwrap_or(0),
            size: parts[1].parse::<i32>().unwrap_or(0),
            protection: parts[2].to_string(),
            flags: parts[3].to_string(),
            fd: fd,
            file_name: file_name,
            offset: offset,
            addr: addr,
        })
    }
    
    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}

#[derive(Debug, Serialize,Deserialize)]
pub struct MMapFilesTracker {
    uuid: String,
}

#[typetag::serde]
impl Trackable for MMapFilesTracker {
    fn track(descs: &mut Descs, _: f64, attrs: Rc<dyn Parsable>) -> Result<Self, String> {

        // Pokusíme se downcastnout na Box<SocketArgs>

        // eprint!("Socket track: \n");

        //eprintln!("descriptors: {:?}\n\n\n", descs);

        let args: Rc<MmapAttrs> = attrs
            .as_any()
            .downcast::<MmapAttrs>()
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
        
        Ok(MMapFilesTracker {
            uuid: uuid.to_string()
        })
    }
}