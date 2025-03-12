use helpers::helpers::HexString;
use registry::registry::Parsable;
use serde::{Deserialize, Serialize};
use std::str::FromStr;


#[derive(Debug, Serialize,Deserialize)]
pub struct AccessArgs {
    file_name: String,
    mode: String,
}

#[typetag::serde]
impl Parsable for AccessArgs {
    fn parse(input: &str) -> Result<Self, String> {
        let parts: Vec<String> = input
                                    .chars()
                                    .filter(|&c| !r#""\"? "#.contains(c))
                                    .collect::<String>()
                                    .split(',')
                                    .map(str::to_string)
                                    .collect::<Vec<String>>();

        if parts.len() < 2 {
            return Err("Invalid number of arguments".into());
        }
        
        let file_name  = HexString::from_str(&parts[0]).unwrap().to_string();

        Ok(AccessArgs {
            file_name: file_name,
            mode: parts[1].to_string(),
        })
    }   
}