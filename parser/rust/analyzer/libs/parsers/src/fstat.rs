// generated by o3-high-mini
use wrappers::parsers::Parsable;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::any::Any;
use helpers::helpers::split_fd_parts;

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct FstatAttrs {
    pub fd: i32,
    pub file_name: String,
    pub statbuf: String,
    pub ret: i32,
}

#[typetag::serde]
impl Parsable for FstatAttrs {
    fn parse(args: &str, result: Option<&str>) -> Result<Self, String> {
        
        let mut attrs = FstatAttrs::default();

        let parts: Vec<String> = args
                                    .chars()
                                    .filter(|&c| !r#""\"? "#.contains(c))
                                    .collect::<String>()
                                    .split(',')
                                    .map(str::to_string)
                                    .collect::<Vec<String>>();

        if parts.len() < 2 {
            return Err("Invalid arguments for fstat".into());
        }
        
        let (fd, file_name) = split_fd_parts(&parts[0]);
        attrs.fd = fd;
        attrs.file_name = file_name;


        if let Some(r) = result {
            attrs.ret = r.trim().parse().unwrap_or(0);
        }
        Ok(attrs)
    }
    
    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}
