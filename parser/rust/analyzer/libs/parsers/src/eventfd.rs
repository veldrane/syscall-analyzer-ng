use helpers::helpers::split_fd_parts;
use wrappers::parsers::Parsable;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::rc::Rc;


#[derive(Debug, Serialize,Deserialize, Default)]
pub struct Eventfd2Args {
    initval: i32,
    flags: i32,
    requested_event_fd: i32,
    requested_event_name: String,
    event_fd: i32,
    event_name: String,
}

#[derive(Debug, Serialize,Deserialize)]
pub struct Eventfd2Results {
    event_fd: i32,
    event_name: String,
}


#[typetag::serde]
impl Parsable for Eventfd2Args {
    fn parse(_: &str, result: Option<&str>) -> Result<Self, String> {

        let mut eventfd2 = Eventfd2Args::default();

        (eventfd2.event_fd, eventfd2.event_name) = match result {
            Some(r) => split_fd_parts(&r),
            None => (0, "".to_string()) 
        };

        Ok(eventfd2)
    }

    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}