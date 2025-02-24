
use std::fs::read_to_string;
use std::process::exit;
use modules::syscall::Syscall;
use modules::registry::SyscallArguments;
use modules::init;
use modules::default;
use regex::Regex;

const BASIC_SYSCALL: &str = r"(?P<timestamp>\d+.\d+)\s(?P<syscall>\w+)\((?P<arguments>.*)\)\s*\=\s(?P<result>.*)\s<(?P<duration>\d+\.\d+)>";


/* Strace parameters for the parser
strace -y -T -ttt -ff -xx -qq -o curl $CMD
*/

// const strace_output: &str = "../../../tests/all.out";

const STRACE_OUTPUT: &str = "../../../tests/syscalls/nginx-all.out";

fn main() {
    let re = Regex::new(BASIC_SYSCALL).unwrap();
    let registry = init::init_registry();


    for line in read_to_string(STRACE_OUTPUT).unwrap().lines() {

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
                let syscall = Syscall {
                    timestamp: fields["timestamp"].to_string(),
                    name: fields["syscall"].to_string(),
                    args: parsed_args,
                    result: fields["result"].to_string(),
                    duration: fields["duration"].to_string(),
                };
                match serde_json::to_string(&syscall) {
                    Ok(json) => println!("{}", json),
                    Err(e) => eprintln!("Chyba při serializaci syscallu {}: {}", &fields["syscall"], e),
                }
            },
            Err(e) => {
                eprintln!("Chyba při parsování syscallu {}: {}\n line: {}", &fields["syscall"], e, line);
            },
        }
    }

    exit(0);
}
