// generated by o3-high-mini
use wrappers::parsers::Parsable;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::any::Any;

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Prlimit64Attrs {
    pub target_pid: i32,
    pub resource: String,
    pub new_limit: String,
    pub old_limit: String,
    pub ret: i32,
}

#[typetag::serde]
impl Parsable for Prlimit64Attrs {
    fn parse(args: &str, result: Option<&str>) -> Result<Self, String> {
        let mut attrs = Prlimit64Attrs::default();
        let parts: Vec<&str> = args.split(',').map(|s| s.trim()).collect();
        if parts.len() < 4 {
            return Err("Invalid arguments for prlimit64".into());
        }
        attrs.target_pid = parts[0].parse().map_err(|e| format!("Failed to parse pid: {}", e))?;
        attrs.resource = parts[1].to_string();
        attrs.new_limit = parts[2].to_string();
        attrs.old_limit = parts[3].to_string();
        if let Some(r) = result {
            attrs.ret = r.trim().parse().unwrap_or(0);
        }
        Ok(attrs)
    }
    
    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}
