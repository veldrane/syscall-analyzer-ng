use std::collections::HashMap;
use registry::registry::Parsable;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize,Deserialize)]
pub struct CloneArgs {
    stack: String,
    flags: String,
    child_tidptr: String,
}

#[derive(Debug, Serialize,Deserialize)]
pub struct CloneResults {
    pub cloned_pid: i32,
}

#[typetag::serde]
impl Parsable for CloneArgs {
    fn parse(input: &str) -> Result<Self, String> {

        let parts: HashMap<String, String> = input
                                    .chars()
                                    .filter(|&c| !r#""\"? "#.contains(c))
                                    .collect::<String>()
                                    .split(',')
                                    .filter_map(|pair| {
                                        let mut attr = pair.split('=');
                                        Some((attr.next()?.to_string(), attr.next()?.to_string()))
                                    })
                                    .collect();

        Ok(CloneArgs {
            stack: parts["child_stack"].clone(),
            flags: parts["flags"].clone(),
            child_tidptr: parts["child_tidptr"].clone(),
        })
    }   
}

#[typetag::serde]
impl Parsable for CloneResults {
    fn parse(input: &str) -> Result<Self, String> {

        let cloned_pid = input.parse::<i32>().map_err(|e| e.to_string())?;

        Ok(CloneResults {
            cloned_pid: cloned_pid,
        })
    }
}