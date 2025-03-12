use helpers::helpers::HexString;
use registry::registry::Parsable;
use serde::{Deserialize, Serialize};
use std::str::FromStr;


#[derive(Debug, Serialize,Deserialize)]
pub struct MprotectArgs {
    addr: String,
    size: String,
    protection: String,
}

#[typetag::serde]
impl Parsable for MprotectArgs {
    fn parse(input: &str) -> Result<Self, String> {
        let parts: Vec<String> = input
                                    .chars()
                                    .filter(|&c| !r#""\"? "#.contains(c))
                                    .collect::<String>()
                                    .split(',')
                                    .map(str::to_string)
                                    .collect::<Vec<String>>();

        if parts.len() < 3{
            return Err("Invalid number of arguments".into());
        }

        Ok(MprotectArgs {
            addr: parts[0].to_string(),
            size: parts[1].to_string(),
            protection: parts[2].to_string(),
        })
    }   
}