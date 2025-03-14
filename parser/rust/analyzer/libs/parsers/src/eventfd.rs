use helpers::helpers::split_fd_parts;
use registry::registry::Parsable;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize,Deserialize, Default)]
pub struct Eventfd2Args {
    initval: i32,
    flags: i32,
}

#[derive(Debug, Serialize,Deserialize)]
pub struct Eventfd2Results {
    event_fd: i32,
    event_name: String,
}


#[typetag::serde]
impl Parsable for Eventfd2Args {
    fn parse(input: &str) -> Result<Self, String> {

        let eventfd2 = Eventfd2Args::default();
        
        Ok(eventfd2)
    }
}

#[typetag::serde]
impl Parsable for Eventfd2Results {
    fn parse(input: &str) -> Result<Self, String> {

        let (fd, file_name ) = split_fd_parts(input);

        Ok(Eventfd2Results{
            event_fd: fd,
            event_name: file_name,
        })
    }   
}