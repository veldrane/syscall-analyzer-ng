use std::fmt::Debug;
use std::fs::read_to_string;
use crate::registry::SyscallArguments;
use regex::Regex;
use serde::{Serialize, Serializer};
use serde_json::value::Value;
use serde_json;
use indexmap::IndexMap;


pub mod examples;
pub mod helpers;
pub mod registry;
pub mod init;
pub mod default;
pub mod mmap;
pub mod open;
pub mod socket;
pub mod accept;
pub mod listen;


const BASIC_SYSCALL: &str = r"(?P<timestamp>\d+.\d+)\s(?P<syscall>\w+)\((?P<arguments>.*)\)\s*\=\s*(?P<results>.*<(?P<duration>\d+\.\d+)>)";


#[derive(Debug)]
struct Syscall {
    timestamp: String,
    name: String,
    args: Box<dyn SyscallArguments>,
}


impl Serialize for Syscall {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Vytvoříme top-level mapu
        let mut map:IndexMap<String, Value> = IndexMap::new();

        map.insert("timestamp".to_string(), Value::String(self.timestamp.clone()));
        map.insert("name".to_string(), Value::String(self.name.clone()));
        // Serializace vnořeného typu do Value
        let args_value = serde_json::to_value(&self.args).unwrap();
        if let Value::Object(args_map) = args_value {
            for (key, value) in args_map.clone() {
                match value {
                    Value::Object(s) => {
                        for (k, v) in s {
                            map.insert(k, v);
                        }
                    },
                    _ => {
                        println!("Unexpected value type: {:?}", value);
                    }
                }
            }
        }
            // Iterace a sloučení hodnot z vnořeného objekt

        // Serializace výsledné mapy
        map.serialize(serializer)
    }
}


/* Strace parameters for the parser
strace -y -T -ttt -ff -xx -qq -o curl $CMD
*/

// const strace_output: &str = "../../../tests/all.out";

const strace_output: &str = "../../../tests/nginx/nginx.41527";

fn main() {
    let registry = init::init_registry();

    //let line = examples::MMAP_FILE;

    for line in read_to_string(strace_output).unwrap().lines() {


        let re = Regex::new(BASIC_SYSCALL).unwrap();

        let fields = if let Some(captures) = re.captures(line) {
            captures
        } else {
            println!("Řádek neodpovídá formátu: {}", line);
            return;
        };
        let result = if let Some(parser) = registry.get(&fields["syscall"]) {
            parser(fields["arguments"].as_ref())
        } else {
            default::DefaultArgs::parse(fields["arguments"].as_ref())
                .map(|v| Box::new(v) as Box<dyn SyscallArguments>)
        };

        match result {
            Ok(parsed_args) => {
                //println!("Parsed syscall args: {:?}", &parsed_args);

                let syscall = Syscall {
                    timestamp: fields["timestamp"].to_string(),
                    name: fields["syscall"].to_string(),
                    args: parsed_args,
                };
                println! ("{}",serde_json::to_string(&syscall).unwrap());
            },
            Err(e) => {
                println!("Chyba při parsování syscallu {}: {}\n line: {}", &fields["syscall"], e, line);
            },
        }
    }
}
