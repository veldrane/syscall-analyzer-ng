use registry::registry::SyscallArguments;
use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct DefaultArgs {
    raw: String,
}

#[typetag::serde]
impl SyscallArguments for DefaultArgs {
    fn parse(input: &str) -> Result<Self, String> {
        Ok(DefaultArgs {
            raw: input.to_string(),
        })
    }
}
