use wrappers::parsers::Parsable;
use serde::{Deserialize, Serialize};
use helpers::converts::hex_serde_u64;


#[derive(Debug, Serialize,Deserialize)]
pub struct MprotectArgs {
    #[serde(with = "hex_serde_u64")]
    addr: u64,
    size: i32,
    protection: String,
}

#[typetag::serde]
impl Parsable for MprotectArgs {
    fn parse(args: &str, _: Option<&str>) -> Result<Self, String> {
        let parts: Vec<String> = args
                                    .chars()
                                    .filter(|&c| !r#""\"? "#.contains(c))
                                    .collect::<String>()    
                                    .split(',')
                                    .map(str::to_string)
                                    .collect::<Vec<String>>();

        if parts.len() < 3{
            return Err("Invalid number of arguments".into());
        }

        let addr = match u64::from_str_radix(&parts[0][2..], 16) {
            Ok(a) => a,
            Err(_) => 0,
        };

        Ok(MprotectArgs {
            addr: addr,
            size: parts[1].parse::<i32>().unwrap_or(0),
            protection: parts[2].to_string(),
        })
    }   
}