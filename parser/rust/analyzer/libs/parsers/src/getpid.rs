use wrappers::parsers::Parsable;
use std::rc::Rc;
use std::any::Any;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct GetpidAttrs {
    pub pid: i32,
}

#[typetag::serde]
impl Parsable for GetpidAttrs {
    fn parse(_: &str, result: Option<&str>) -> Result<Self, String> {
        let mut attrs = GetpidAttrs::default();
        if let Some(r) = result {
            attrs.pid = r.trim().parse().unwrap_or(0);
        }
        Ok(attrs)
    }
    
    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}
