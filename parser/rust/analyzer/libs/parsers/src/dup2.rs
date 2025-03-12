use helpers::helpers::split_fd_parts;
use registry::registry::Parsable;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize,Deserialize)]
pub struct Dup2Args {
    old_fd: i32,
    old_file_name: String,
    requested_fd: i32,
    requested_file_name: String,
}

#[derive(Debug, Serialize,Deserialize)]
pub struct Dup2Results {
    fd: i32,
    file_name: String,
}

#[typetag::serde]
impl Parsable for Dup2Results {
        fn parse(input: &str) -> Result<Self, String> {
            
            let parts: Vec<&str> = input
                                        .split(' ')
                                        .collect();
    
    
            if parts[0] == "-1" {
                return Err("Error opening file".into());
            }
    
            let (fd, file_name) = split_fd_parts(&parts[0]);
    
            Ok(Dup2Results {
                fd: fd,
                file_name: file_name,
            })
        }
}

#[typetag::serde]
impl Parsable for Dup2Args {
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
        
        let (old_fd, old_file_name ) = split_fd_parts(&parts[0]);
        let (requested_fd, requested_file_name ) = split_fd_parts(&parts[1]);


        Ok(Dup2Args {
            old_fd: old_fd,
            old_file_name: old_file_name,
            requested_fd: requested_fd,
            requested_file_name: requested_file_name,
        })
    }   
}