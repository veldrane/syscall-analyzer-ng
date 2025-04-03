// generated by o3-high-mini
use wrappers::parsers::Parsable;
use std::rc::Rc;
use std::any::Any;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct FtruncateAttrs {
    pub fd: i32,
    pub length: i64,
    pub ret: i32,
}

#[typetag::serde]
impl Parsable for FtruncateAttrs {
    fn parse(args: &str, result: Option<&str>) -> Result<Self, String> {
        let mut attrs = FtruncateAttrs::default();
        let parts: Vec<&str> = args.split(',').map(|s| s.trim()).collect();
        if parts.len() < 2 {
            return Err("Invalid arguments for ftruncate".into());
        }
        attrs.fd = parts[0].parse().map_err(|e| format!("Failed to parse fd: {}", e))?;
        attrs.length = parts[1].parse().map_err(|e| format!("Failed to parse length: {}", e))?;
        if let Some(r) = result {
            attrs.ret = r.trim().parse().unwrap_or(0);
        }
        Ok(attrs)
    }
    
    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}
