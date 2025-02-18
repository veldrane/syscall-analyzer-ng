use crate::registry::SyscallArguments;

#[derive(Debug)]
pub struct OpenArguments {
    dirfd: String,
    object: String,
    mode: String,
}

impl SyscallArguments for OpenArguments {
    fn parse(input: &str) -> Result<Self, String> {
        
        let parts: Vec<String> = input
                                    .chars()
                                    .filter(|&c| !r#""\"? "#.contains(c))
                                    .collect::<String>()
                                    .split(',')
                                    .map(str::to_string)
                                    .collect::<Vec<String>>();
        if parts.len() != 3 {
            return Err("Invalid number of arguments".into());
        }
        Ok(OpenArguments {
            dirfd: parts[0].to_string(),
            object: parts[1].to_string(),
            mode: parts[2].to_string()
    })
}
}