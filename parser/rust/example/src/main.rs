use std::fmt::Debug;
use crate::registry::SyscallArguments;
use regex::Regex;

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


const BASIC_SYSCALL: &str = r"(?P<timestamp>\d+.\d+)\s(?P<syscall>\w+)\((?P<arguments>.*)\)\s+\=\s(?P<results>.*<(?P<duration>\d+\.\d+)>)";


#[derive(Debug)]
struct Syscall {
    timestamp: String,
    name: String,
    args: Option<Box<dyn SyscallArguments>>,
}





fn main() {
    let registry = init::init_registry();

    let line = examples::LISTEN;

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
            println!("Syscall {} byl úspěšně parsován.", &fields["syscall"]);
            // Výpis detailů pomocí formátování Debug
            // syscall.args = Some(parsed_args);
           println!("Raw syscall args: {:?}", &fields["arguments"]);
           println!("Parsed syscall args: {:?}", &parsed_args);

        },
        Err(e) => {
            println!("Chyba při parsování syscallu {}: {}", &fields["syscall"], e);
        },
    }
}
