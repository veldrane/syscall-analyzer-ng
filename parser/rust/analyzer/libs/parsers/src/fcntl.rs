use helpers::helpers::split_fd_parts;
use registry::registry::Parsable;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;


#[skip_serializing_none]
#[derive(Debug, Serialize,Deserialize, Default)]
pub struct FcntlArgs {
    fd: Option<i32>,
    file_name: Option<String>,
    socket_name: Option<String>,
    socket_fd: Option<i32>,
    operation: String,
    opt_arg: String,
}


#[typetag::serde]
impl Parsable for FcntlArgs {
    fn parse(input: &str) -> Result<Self, String> {

        let mut fcntl_args: FcntlArgs = FcntlArgs::default();

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

        fcntl_args.opt_arg = if let Some(opt) = parts.get(2) {
            opt.to_string()
        } else {
            "".to_string()
        };

        let (descriptor, name ) = split_fd_parts(&parts[0]);

        match name.as_str() {
            s if s.contains("socket:") => {
                fcntl_args.socket_name = Some(name);
                fcntl_args.socket_fd = Some(descriptor);
            },
            _ => {
                fcntl_args.fd = Some(descriptor);
                fcntl_args.file_name = Some(name);
            }
        };

        fcntl_args.operation = parts[1].to_string();

        return  Ok(fcntl_args);
    }   
}