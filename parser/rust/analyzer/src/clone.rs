use std::collections::HashMap;

use crate::registry::SyscallArguments;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize,Deserialize)]
pub struct CloneArgs {
    stack: String,
    flags: String,
    child_tidptr: String,
}


#[typetag::serde]
impl SyscallArguments for CloneArgs {
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

