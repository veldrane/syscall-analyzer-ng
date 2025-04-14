use std::{fs::read_to_string, collections::HashMap, error::Error, rc::Rc};
use parsers::syscall::Syscall;
use registry::registry::RegistryEntry;
use trackers::{fd_table::Descs, archive::Archive};
use regex::Regex;
use crate::{helpers::*, logging::Logger, file_output::DocumentSaver};

pub fn process_pid(
    trace_dir: &str,
    pid: i32,
    timestamp: f64,
    regex: &Regex,
    registry: &HashMap<String, RegistryEntry>,
    archive: &mut Archive,
    mut id_counter: i32,
    worklist: &mut Vec<i32>,
    log: &Logger,
    backend: &DocumentSaver
) -> Result<i32, Box<dyn Error>> {

    let mut descs = match archive.get_descs(pid) {
        Some(d) => d.clone(),
        None => Descs::with_std_fds(timestamp),

    };

    let trace_path = format!("{}/sshd.{}", trace_dir, pid);
    let content = read_to_string(&trace_path)?;
    for line in content.lines() {
        id_counter = process_line(line, pid,  regex, registry, &mut descs, archive, worklist, id_counter, log, backend)?;
    }

    Ok(id_counter)

}


pub fn process_line(
    line: &str,
    pid: i32,
    regex: &Regex,
    registry: &HashMap<String, RegistryEntry>,
    descs: &mut Descs,
    archive: &mut Archive,
    worklist: &mut Vec<i32>,
    id_counter: i32,
    log: &Logger,
    backend: &DocumentSaver
) -> Result<i32, Box<dyn Error>> {

    //let fields = match get_fields(line, &regex).ok_or(log.error(format!("Error parsing  cosi line"))) {
    //    Ok(fields) => fields,
    //    _ => {
    //        return Ok(id_counter);
    //    },
    //};

    let fields = match get_fields(line, &regex) {
        Some(f) => f,
        None => {
            log.error(format!("parsing basic line: {}", line));
            return Ok(id_counter);
        }
    };




    let (timestamp, name, args, result, duration) = get_basic(&fields);
    let parsers = registry.get(name);
    let attributes = match do_parse(parsers, args, result, log) {
        Ok(parsed_attributes) => parsed_attributes,
        Err(_) => {
            log.error(format!("parsing syscall attributes, line {}", line));
            return Ok(id_counter);
        },
    };
    let trackers = do_track(parsers, descs, timestamp, Rc::clone(&attributes));


    if name == "clone" {
        if let Some(cloned_pid) = get_cloned_pid(Rc::clone(&attributes)) {
            archive.add_descs(cloned_pid, descs.clone_alive());
            worklist.push(cloned_pid);
        } else {
            log.error(format!("parsing clone attributes, line {}", line));
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

    let document = serde_json::to_string(&syscall)?;

    DocumentSaver::send(&backend, document);
    Ok(new_id)

}