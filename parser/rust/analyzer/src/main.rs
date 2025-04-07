use core::time;
use std::fs::read_to_string;
use std::process::exit;
use parsers::syscall::Syscall;
use parsers::clone::CloneAttrs;
use registry::registry::RegistryEntry;
use parsers::default;
use trackers::fd_table::Descs;
use trackers::archive::Archive;
use wrappers::parsers::Parsable;
use wrappers::trackers::Trackable;
use std::collections::HashMap;

use modules::init;
use modules::inputs;
use regex::Regex;
use std::rc::Rc;


const BASIC_SYSCALL: &str = r"(?P<timestamp>\d+.\d+)\s(?P<name>\w+)\((?P<arguments>.*)\)\s*\=\s(?P<result>.*)\s<(?P<duration>\d+\.\d+)>";


/* Strace parameters for the parser
strace -y -T -ttt -ff -xx -qq -o curl $CMD
*/

// const STRACE_OUTPUT: &str = "../../../tests/curl/curl.38945";
// const STRACE_OUTPUT: &str = "../../../tests/sshd/sshd.8797";
// const STRACE_OUTPUT: &str = "../../../tests/syscalls/nginx-all.out";

const STRACE_DIR: &str = "../../../tests/sshd";

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let registry = init::build_registry();

    run(&registry)?;
    Ok(())
}


fn run(registry: &HashMap<String, RegistryEntry>) -> Result<(), Box<dyn std::error::Error>> {

    let re = Regex::new(BASIC_SYSCALL)?;
    //let mut descs = Descs::with_std_fds(1739965813.133382);
    let mut archive = Archive::new();
    let (trace, ts) = inputs::find_first(STRACE_DIR).expect("First trace file not found");

    let mut id = 0;
    let pid = 8797;

    let mut descs = get_descs(&archive, pid);


    for (pid, descs) in archive.iter() {
        println!("pid {}, descs: {:?}", pid, descs);
    }


    
    for line in read_to_string(&trace)?.lines() {

        id += 1;
        let fields = get_fields(line, &re).ok_or("Error parsing line")?;

        let (timestamp, name, args, result, duration) = get_basic(&fields);
        let parsers = registry.get(name);
    
        let attributes = match do_parse(parsers, args, result) {
            Ok(parsed_attributes) => parsed_attributes,
            Err(e) => {
                eprintln!("Error parsing syscall attributes: {} with error: {}", &line, e);
                continue;
            },
        };
        let trackers = do_track(parsers, &mut descs, timestamp, Rc::clone(&attributes));

        if name == "clone" {
            let cloned_pid = get_cloned_pid(Rc::clone(&attributes)).ok_or("No cloned pid")?;
            archive.add_descs(cloned_pid, Descs::with_std_fds(1739965813.133382));
        }


        let syscall = Syscall {
            pid: pid,
            id: &id,
            timestamp: &timestamp,
            name: name,
            attributes: attributes,
            trackers: trackers,
            result: result.unwrap_or(""),
            duration: duration,
        };

        println!("{}", serde_json::to_string(&syscall).unwrap());
    }

    exit(0);

}


fn get_descs(archive: &Archive, pid: i32) -> Descs {
    
    match archive.get_descs(pid) {
        Some(descs) => descs,
        None => Descs::with_std_fds(1739965813.133382),
    }
}

fn get_cloned_pid(attributes: Rc<dyn Parsable>) -> Option<i32> {
    if let Some(attrs) = attributes.as_any().downcast_ref::<CloneAttrs>() {
        Some(attrs.cloned_pid)
    } else {
        None
    }
}

fn get_basic<'l>(fields: &'l regex::Captures) -> (f64, &'l str, &'l str, Option<&'l str>, &'l str) {
    let timestamp = fields["timestamp"].parse::<f64>().unwrap();
    let name = fields["name"].as_ref();
    let args = fields["arguments"].as_ref();
    let result = Some(fields["result"].as_ref());
    let duration = fields["duration"].as_ref();

    (timestamp, name, args, result, duration)
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

