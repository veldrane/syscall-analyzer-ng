use wrappers::parsers::Parsable;
use serde::{Serialize, Deserialize};
use std::any::Any;
use std::rc::Rc;

#[derive(Debug,Serialize,Deserialize)]
pub struct DefaultArgs {
    raw: String,
}

#[typetag::serde]
impl Parsable for DefaultArgs {
    fn parse(args: &str, _: Option<&str>) -> Result<Self, String> {
        Ok(DefaultArgs {
            raw: args.to_string(),
        })
    }

    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }

}
