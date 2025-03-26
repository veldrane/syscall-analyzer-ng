use wrappers::parsers::Parsable;
use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct DefaultArgs {
    raw: String,
}

#[typetag::serde]
impl Parsable for DefaultArgs {
    fn parse(args: &str, _: Option<&str>) -> Result<Self, String> {
        Ok(DefaultArgs {
            raw: args.to_string(),
        })
    }
}
