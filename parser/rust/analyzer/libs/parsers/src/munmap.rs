use serde::{Deserialize, Serialize};
use wrappers::parsers::Parsable;
use helpers::converts::hex_serde_u64;
use std::any::Any;
use std::rc::Rc;

#[derive(Debug, Serialize,Deserialize)]
pub struct MunmapArgs {
    #[serde(with = "hex_serde_u64")]
    addr: u64,
    size: i32,
}


#[typetag::serde]
impl Parsable for MunmapArgs {
    
    fn parse(args: &str, _: Option<&str>) -> Result<Self, String> {


        let parts: Vec<String> = args
                                    .chars()
                                    .filter(|&c| !r#""\"? "#.contains(c))
                                    .collect::<String>()
                                    .split(',')
                                    .map(str::to_string)
                                    .collect::<Vec<String>>();

        let addr = match u64::from_str_radix(&parts[0][2..], 16) {
            Ok(a) => a,
            Err(_) => 0,
        };

        Ok(MunmapArgs {
            addr: addr,
            size: parts[1].parse::<i32>().unwrap(),
        })
    }

    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}