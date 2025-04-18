// generated by o3-high-mini
use wrappers::parsers::Parsable;
use std::rc::Rc;
use std::any::Any;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct FlistxattrAttrs {
    pub fd: i32,
    pub list: String,
    pub size: usize,
    pub ret: i32,
}

#[typetag::serde]
impl Parsable for FlistxattrAttrs {
    fn parse(args: &str, result: Option<&str>) -> Result<Self, String> {
        let mut attrs = FlistxattrAttrs::default();
        let parts: Vec<&str> = args.split(',').map(|s| s.trim()).collect();
        if parts.len() < 3 {
            return Err("Invalid arguments for flistxattr".into());
        }
        attrs.fd = parts[0].parse().map_err(|e| format!("Failed to parse fd: {}", e))?;
        attrs.list = parts[1].to_string();
        attrs.size = parts[2].parse().map_err(|e| format!("Failed to parse size: {}", e))?;
        if let Some(r) = result {
            attrs.ret = r.trim().parse().unwrap_or(0);
        }
        Ok(attrs)
    }
    
    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}
