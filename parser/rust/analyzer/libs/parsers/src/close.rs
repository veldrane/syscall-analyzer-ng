use helpers::helpers::split_fd_parts;
use wrappers::parsers::Parsable;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::rc::Rc;


#[derive(Debug, Serialize,Deserialize)]
pub struct CloseArgs {
    fd: i32,
    file_name: String,
}


#[typetag::serde]
impl Parsable for CloseArgs {
    fn parse(args: &str, _: Option<&str>) -> Result<Self, String> {

        let (fd, file_name ) = split_fd_parts(&args);

        Ok(CloseArgs {
            fd: fd,
            file_name: file_name,
        })
    }

    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }  
}