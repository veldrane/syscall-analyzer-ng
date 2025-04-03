// generated by o3-high-mini
use wrappers::parsers::Parsable;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::any::Any;

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct ArchPrctlAttrs {
    pub code: String,
    pub addr: String,
    pub ret: i32,
}

#[typetag::serde]
impl Parsable for ArchPrctlAttrs {
    fn parse(args: &str, result: Option<&str>) -> Result<Self, String> {
        let mut attrs = ArchPrctlAttrs::default();
        let parts: Vec<&str> = args.split(',').map(|s| s.trim()).collect();
        if parts.len() < 2 {
            return Err("Invalid arguments for arch_prctl".into());
        }
        attrs.code = parts[0].to_string();
        attrs.addr = parts[1].to_string();
        if let Some(r) = result {
            attrs.ret = r.trim().parse().unwrap_or(0);
        }
        Ok(attrs)
    }
    
    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}
