use std::fs::read_to_string;
use std::process::exit;
use parsers::syscall::Syscall;
use registry::registry::Register;
use parsers::default;
use trackers::descriptors::Descs;
use wrappers::parsers::Parsable;
use std::collections::HashMap;

use modules::init;
use regex::Regex;


const BASIC_SYSCALL: &str = r"(?P<timestamp>\d+.\d+)\s(?P<syscall>\w+)\((?P<arguments>.*)\)\s*\=\s(?P<result>.*)\s<(?P<duration>\d+\.\d+)>";


/* Strace parameters for the parser
strace -y -T -ttt -ff -xx -qq -o curl $CMD
*/

const STRACE_OUTPUT: &str = "../../../tests/curl/curl.38945";
// const STRACE_OUTPUT: &str = "../../../tests/syscalls/nginx-all.out";

fn main() -> Result<(), Box<dyn std::error::Error>> {


    //let registry = Register::new();

    let registry = init::init_registry();

    run(&registry)?;
    Ok(())
}


fn run(registry: &HashMap<String, Register>) -> Result<(), Box<dyn std::error::Error>> {

    let mut descriptors = Descs::init_empty_process("1739965813.133382".to_string());

    let re = Regex::new(BASIC_SYSCALL)?;

    let mut id = 0;
    for line in read_to_string(STRACE_OUTPUT)?.lines() {

        id += 1;
        let fields = match re.captures(line) {
            Some(captures) => captures,
            None => {
                eprintln!("Řádek neodpovídá formátu: {}", line);
                continue;
            },
        };

        let parsers = registry.get(&fields["syscall"]);

        let parsed_attributes = if let Some(parsers) = parsers {
            (parsers.attributes)(fields["arguments"].as_ref(), Some(fields["result"].as_ref()))
        } else {
            default::DefaultArgs::parse(fields["arguments"].as_ref(), None)
                .map(|v| Box::new(v) as Box<dyn Parsable>)
        };


        let attributes = match parsed_attributes {
            Ok(parsed_args) => parsed_args,
            Err(e) => {
                eprintln!("Chyba při parsování syscallu {}: {}\n line: {}", &fields["syscall"], e, line);
                continue;
            },
        };

        let syscall = Syscall {
            id: &id,
            timestamp: fields["timestamp"].as_ref(),
            name: fields["syscall"].as_ref(),
            attributes: attributes,
            result: fields["result"].as_ref(),
            duration: fields["duration"].as_ref(),
        };

        println!("{}", serde_json::to_string(&syscall).unwrap());
    }

    exit(0);

}
