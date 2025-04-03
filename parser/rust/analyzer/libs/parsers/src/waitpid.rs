// Generated file, do not edit

use wrappers::parsers::Parsable;
use std::rc::Rc;
use std::any::Any;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize,)]
pub struct WaitpidAttrs {
    pub pid: i32,
    pub options: i32,
    pub ret: i32,
}

#[typetag::serde]
impl Parsable for WaitpidAttrs {
    fn parse(args: &str, result: Option<&str>) -> Result<Self, String> {
        let mut attrs = WaitpidAttrs::default();
        let parts: Vec<&str> = args.split(',').map(|s| s.trim()).collect();
        if parts.len() < 2 {
            return Err("Invalid arguments for waitpid".to_string());
        }
        attrs.pid = parts[0]
            .parse()
            .map_err(|e| format!("Failed to parse pid: {}", e))?;
        attrs.options = parts[1]
            .parse()
            .map_err(|e| format!("Failed to parse options: {}", e))?;
        if let Some(r) = result {
            attrs.ret = r.trim().parse().unwrap_or(0);
        }
        Ok(attrs)
    }
    
    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}
