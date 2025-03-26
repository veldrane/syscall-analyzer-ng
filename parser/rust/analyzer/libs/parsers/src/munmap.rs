use serde::{Deserialize, Serialize};
use wrappers::parsers::Parsable;

#[derive(Debug, Serialize,Deserialize)]
pub struct MunmapArgs {
    addr: String,
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

        Ok(MunmapArgs {
            addr: parts[0].to_string(),
            size: parts[1].parse::<i32>().unwrap(),
        })
    }
}