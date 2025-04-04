// Generated file, do not edit - revised and added filename 03.04.2025
use wrappers::parsers::Parsable;
use std::rc::Rc;
use std::any::Any;
use serde::{Deserialize, Serialize};
use helpers::helpers::HexString;
use std::str::FromStr;

#[derive(Default, Debug, Serialize, Deserialize )]
pub struct ExecveAttrs {
    pub file_name: String,
    pub argv: Vec<String>,
    pub envp: Vec<String>,
    pub ret: i32,
}

#[typetag::serde]
impl Parsable for ExecveAttrs {
    fn parse(args: &str, result: Option<&str>) -> Result<Self, String> {
        // Očekáváme formát: "filename, argv=[arg1 arg2 ...], envp=[env1 env2 ...]"
        let mut attrs = ExecveAttrs::default();

        let parts: Vec<String> = args
        .chars()
        .filter(|&c| !r#""\"? "#.contains(c))
        .collect::<String>()
        .split(',')
        .map(str::to_string)
        .collect::<Vec<String>>();
        
        if parts.len() < 3 {
            return Err("Invalid arguments for execve".to_string());
        }

        attrs.file_name = HexString::from_str(&parts[0]).unwrap().to_string();

        // Zpracování argv (očekáváme řetězec typu argv=[...])
        if let Some(start) = parts[1].find('[') {
            if let Some(end) = parts[1].find(']') {
                let list = &parts[1][start+1..end];
                attrs.argv = list
                    .split_whitespace()
                    .map(|s| s.trim_matches('"').to_string())
                    .collect();
            }
        }

        // Zpracování envp (očekáváme řetězec typu envp=[...])
        if let Some(start) = parts[2].find('[') {
            if let Some(end) = parts[2].find(']') {
                let list = &parts[2][start+1..end];
                attrs.envp = list
                    .split_whitespace()
                    .map(|s| s.trim_matches('"').to_string())
                    .collect();
            }
        }

        if let Some(r) = result {
            attrs.ret = r.trim().parse().unwrap_or(0);
        }
        Ok(attrs)
    }
    
    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}
