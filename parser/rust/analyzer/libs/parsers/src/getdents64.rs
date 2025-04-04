// generated by o3-high-mini
use wrappers::parsers::Parsable;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::any::Any;

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Getdents64Attrs {
    pub fd: i32,
    pub dirp: String,
    pub count: usize,
    pub ret: i32,
}

#[typetag::serde]
impl Parsable for Getdents64Attrs {
    fn parse(args: &str, result: Option<&str>) -> Result<Self, String> {
        let mut attrs = Getdents64Attrs::default();
        let parts: Vec<&str> = args.split(',').map(|s| s.trim()).collect();
        if parts.len() < 3 {
            return Err("Invalid arguments for getdents64".into());
        }
        attrs.fd = parts[0].parse().map_err(|e| format!("Failed to parse fd: {}", e))?;
        attrs.dirp = parts[1].to_string();
        attrs.count = parts[2].parse().map_err(|e| format!("Failed to parse count: {}", e))?;
        if let Some(r) = result {
            attrs.ret = r.trim().parse().unwrap_or(0);
        }
        Ok(attrs)
    }
    
    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}
