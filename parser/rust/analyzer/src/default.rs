use crate::registry::SyscallArguments;

#[derive(Debug)]
pub struct DefaultArgs {
    raw: String,
}

impl SyscallArguments for DefaultArgs {
    fn parse(input: &str) -> Result<Self, String> {
        Ok(DefaultArgs {
            raw: input.to_string(),
        })
    }
}
