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
use std::error::Error;

use modules::init;
use modules::inputs;
use regex::Regex;
use std::rc::Rc;


const BASIC_SYSCALL: &str = r"(?P<timestamp>\d+.\d+)\s(?P<name>\w+)\((?P<arguments>.*)\)\s*\=\s(?P<result>.*)\s<(?P<duration>\d+\.\d+)>";


/* Strace parameters for the parser
strace -y -T -ttt -ff -xx -qq -o curl $CMD
*/

const STRACE_DIR: &str = "/home/veldrane/Bitbucket/private/syscall-analyzer-ng/tests/sshd";


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let registry = init::build_registry();

    run(&registry)?;
    Ok(())
}


fn run(registry: &HashMap<String, RegistryEntry>) -> Result<(), Box<dyn std::error::Error>> {

    let basic_regex = Regex::new(BASIC_SYSCALL)?;
    let (pid, init_timestamp) = inputs::find_first(STRACE_DIR).expect("First trace file not found");
    let mut archive = Archive::new();
    archive.add_descs(pid, Descs::with_std_fds(init_timestamp));

    let mut worklist: Vec<i32> = [pid].to_vec();
    let mut id_counter = 0;
    
    while let Some(pid) = worklist.pop() {
        id_counter = process_pid(
            pid,
            init_timestamp,
            &basic_regex,
            registry,
            &mut archive,
            id_counter,
            &mut worklist,
        )?;
    }


    exit(0);
}


fn process_pid(
    pid: i32,
    timestamp: f64,
    regex: &Regex,
    registry: &HashMap<String, RegistryEntry>,
    archive: &mut Archive,
    mut id_counter: i32,
    worklist: &mut Vec<i32>,
) -> Result<i32, Box<dyn Error>> {

    let mut descs = match archive.get_descs(pid) {
        Some(d) => d.clone(),
        None => Descs::with_std_fds(timestamp),

    };
    let trace_path = format!("{}/sshd.{}", STRACE_DIR, pid);
    let content = read_to_string(&trace_path)?;

    for line in content.lines() {
        id_counter = process_line(line, pid,  regex, registry, &mut descs, archive, worklist, id_counter)?;
    }

    Ok(id_counter)

}


fn process_line(
    line: &str,
    pid: i32,
    regex: &Regex,
    registry: &HashMap<String, RegistryEntry>,
    descs: &mut Descs,
    archive: &mut Archive,
    worklist: &mut Vec<i32>,
    id_counter: i32,
) -> Result<i32, Box<dyn Error>> {

    let fields = match get_fields(line, &regex).ok_or("Error parsing line") {
        Ok(fields) => fields,
        _ => {
            return Ok(id_counter);
        },
    };
    let (timestamp, name, args, result, duration) = get_basic(&fields);
    let parsers = registry.get(name);
    let attributes = match do_parse(parsers, args, result) {
        Ok(parsed_attributes) => parsed_attributes,
        Err(_) => {
            return Ok(id_counter);
        },
    };
    let trackers = do_track(parsers, descs, timestamp, Rc::clone(&attributes));


    if name == "clone" {
        if let Some(cloned_pid) = get_cloned_pid(Rc::clone(&attributes)) {
            archive.add_descs(cloned_pid, Descs::with_std_fds(timestamp));
            worklist.push(cloned_pid);
        } else {
            eprintln!("Error parsing clone attributes, line {}", line);
            return Ok(id_counter);
        }
    }

    let new_id = id_counter + 1;
    let syscall = Syscall::new()
        .with_pid(&pid)
        .with_id(&new_id)
        .with_timestamp(&timestamp)
        .with_name(name)
        .with_attributes(Rc::clone(&attributes))
        .with_trackers(trackers)
        .with_result(result.unwrap_or(""))
        .with_duration(&duration)
        .build();

    println!("{}", serde_json::to_string(&syscall)?);
    Ok(new_id)

}

fn get_cloned_pid(attributes: Rc<dyn Parsable>) -> Option<i32> {
    if let Some(attrs) = attributes.as_any().downcast_ref::<CloneAttrs>() {
        Some(attrs.cloned_pid)
    } else {
        None
    }
}

fn get_basic<'l>(fields: &'l regex::Captures) -> (f64, &'l str, &'l str, Option<&'l str>,f64) {
    let timestamp = fields["timestamp"].parse::<f64>().unwrap();
    let name = fields["name"].as_ref();
    let args = fields["arguments"].as_ref();
    let result = Some(fields["result"].as_ref());
    let duration = fields["duration"].parse::<f64>().unwrap();

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

