use wrappers::parsers::Parsable;
use serde::{Serialize, Deserialize};
use std::any::Any;
use std::rc::Rc;

#[derive(Debug,Serialize,Deserialize)]
pub struct RawAttrs {
    raw: String,
}

#[typetag::serde]
impl Parsable for RawAttrs {
    fn parse(args: &str, _: Option<&str>) -> Result<Self, String> {
        Ok(RawAttrs {
            raw: args.to_string(),
        })
    }

    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }

}
