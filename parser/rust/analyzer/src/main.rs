use std::fs::read_to_string;
use std::process::exit;
use parsers::syscall::Syscall;
use registry::registry::RegistryEntry;
use parsers::default;
use trackers::fd_table::Descs;
use wrappers::parsers::Parsable;
use wrappers::trackers::Trackable;
use std::collections::HashMap;

use modules::init;
use regex::Regex;
use std::rc::Rc;


const BASIC_SYSCALL: &str = r"(?P<timestamp>\d+.\d+)\s(?P<syscall>\w+)\((?P<arguments>.*)\)\s*\=\s(?P<result>.*)\s<(?P<duration>\d+\.\d+)>";


/* Strace parameters for the parser
strace -y -T -ttt -ff -xx -qq -o curl $CMD
*/

// const STRACE_OUTPUT: &str = "../../../tests/curl/curl.38945";
const STRACE_OUTPUT: &str = "../../../tests/sshd/sshd.8797";
// const STRACE_OUTPUT: &str = "../../../tests/syscalls/nginx-all.out";

fn main() -> Result<(), Box<dyn std::error::Error>> {


    //let registry = Register::new();

    let registry = init::build_registry();

    run(&registry)?;
    Ok(())
}


fn run(registry: &HashMap<String, RegistryEntry>) -> Result<(), Box<dyn std::error::Error>> {

    let re = Regex::new(BASIC_SYSCALL)?;
    let mut descs = Descs::with_std_fds(1739965813.133382);
    let mut id = 0;

    for line in read_to_string(STRACE_OUTPUT)?.lines() {

        id += 1;
        let fields = get_fields(line, &re).ok_or("Error parsing line")?;

        let timestamp = fields["timestamp"].parse::<f64>()?;
        let syscall: &str = fields["syscall"].as_ref();
        let args = fields["arguments"].as_ref();
        let result = Some(fields["result"].as_ref());

        let parsers = registry.get(syscall);
    
        let attributes = match do_parse(parsers, args, result) {
            Ok(parsed_attributes) => parsed_attributes,
            Err(e) => {
                eprintln!("Error parsing syscall attributes: {} with error: {}", &line, e);
                continue;
            },
        };
        let trackers = do_track(parsers, &mut descs, timestamp, Rc::clone(&attributes));


        let syscall = Syscall {
            id: &id,
            timestamp: &timestamp,
            name: fields["syscall"].as_ref(),
            attributes: attributes,
            trackers: trackers,
            result: fields["result"].as_ref(),
            duration: fields["duration"].as_ref(),
        };

        println!("{}", serde_json::to_string(&syscall).unwrap());
    }

    exit(0);

}


fn get_fields<'a, 're>(line: &'a str, re: &'re Regex) -> Option<regex::Captures<'a>> {
    match re.captures(line) {
        Some(captures) => Some(captures),
        None => None,
    }
}

fn do_parse(parsers: Option<&RegistryEntry>, args: &str, result: Option<&str>) -> Result<Rc<dyn Parsable>, String> {

    let parsed_attributes = if let Some(parsers) = parsers {
        (parsers.attributes)(args, result)
    } else {
        default::RawAttrs::parse(args, None)
            .map(|v| Rc::new(v) as Rc<dyn Parsable>)
    };


    let attributes = match parsed_attributes {
        Ok(parsed_args) => parsed_args,
        Err(e) => {
            eprintln!("Chyba při parsování syscallu");
            return Err(e);
        },
    };

    Ok(attributes)
}

fn do_track(parsers: Option<&RegistryEntry>, descs: &mut Descs, timestamp: f64, attributes: Rc<dyn Parsable>) -> Option<Box<dyn Trackable>> {

    if let Some(parsers) = parsers {
        match &parsers.trackers {
            Some(trackers) => trackers(descs, timestamp, attributes),
            None => Err("No trackers".to_string()),
        }
    } else {
        Err("()".to_string())
    }.ok()
}
