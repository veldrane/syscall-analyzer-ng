// generated by o3-high-mini
use wrappers::parsers::Parsable;
use std::rc::Rc;
use std::any::Any;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct ForkAttrs {
    pub ret: i32,
}

#[typetag::serde]
impl Parsable for ForkAttrs {
    fn parse(_args: &str, result: Option<&str>) -> Result<Self, String> {
        let mut attrs = ForkAttrs::default();
        if let Some(r) = result {
            attrs.ret = r.trim().parse().unwrap_or(0);
        }
        Ok(attrs)
    }
    
    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}
