use std::collections::HashMap;
//use registry::registry::Parsable;
use serde::{Deserialize, Serialize};
use wrappers::parsers::Parsable;
use std::any::Any;
use std::rc::Rc;

#[derive(Debug, Serialize,Deserialize)]
pub struct CloneAttrs {
    stack: String,
    flags: String,
    child_tidptr: String,
    cloned_pid: i32,
}

#[typetag::serde]
impl Parsable for CloneAttrs {
    fn parse(args: &str, result: Option<&str>) -> Result<Self, String> {

        let parts: HashMap<String, String> = args
                                    .chars()
                                    .filter(|&c| !r#""\"? "#.contains(c))
                                    .collect::<String>()
                                    .split(',')
                                    .filter_map(|pair| {
                                        let mut attr = pair.split('=');
                                        Some((attr.next()?.to_string(), attr.next()?.to_string()))
                                    })
                                    .collect();

        let cloned_pid = match result {
            Some(r) => r.parse::<i32>().map_err(|e| e.to_string())?,
            None => 0
        };

        Ok(CloneAttrs {
            stack: parts["child_stack"].clone(),
            flags: parts["flags"].clone(),
            child_tidptr: parts["child_tidptr"].clone(),
            cloned_pid: cloned_pid,
        })
    }

    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }   
}
