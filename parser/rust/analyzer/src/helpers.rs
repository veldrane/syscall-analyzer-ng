use std::rc::Rc;
use parsers::{clone::CloneAttrs, default};
use registry::registry::RegistryEntry;
use trackers::fd_table::Descs;
use wrappers::{parsers::Parsable, trackers::Trackable};
use regex::Regex;

use crate::logging::Logger;

pub fn get_cloned_pid(attributes: Rc<dyn Parsable>) -> Option<i32> {
    if let Some(attrs) = attributes.as_any().downcast_ref::<CloneAttrs>() {
        Some(attrs.cloned_pid)
    } else {
        None
    }
}

pub fn get_basic<'l>(fields: &'l regex::Captures) -> (f64, &'l str, &'l str, Option<&'l str>,f64) {
    let timestamp = fields["timestamp"].parse::<f64>().unwrap();
    let name = fields["name"].as_ref();
    let args = fields["arguments"].as_ref();
    let result = Some(fields["result"].as_ref());
    let duration = fields["duration"].parse::<f64>().unwrap();

    (timestamp, name, args, result, duration)
}

pub fn get_fields<'a, 're>(line: &'a str, re: &'re Regex) -> Option<regex::Captures<'a>> {
    match re.captures(line) {
        Some(captures) => Some(captures),
        None => None,
    }
}

pub fn do_parse(parsers: Option<&RegistryEntry>, args: &str, result: Option<&str>, log: &Logger) -> Result<Rc<dyn Parsable>, String> {

    let parsed_attributes = if let Some(parsers) = parsers {
        (parsers.attributes)(args, result)
    } else {
        default::RawAttrs::parse(args, None)
            .map(|v| Rc::new(v) as Rc<dyn Parsable>)
    };


    let attributes = match parsed_attributes {
        Ok(parsed_args) => parsed_args,
        Err(e) => {
            log.error(format!("parsing syscall arguments: {}", e));
            return Err(e);
        },
    };

    Ok(attributes)
}

pub fn do_track(parsers: Option<&RegistryEntry>, descs: &mut Descs, timestamp: f64, attributes: Rc<dyn Parsable>) -> Option<Box<dyn Trackable>> {

    if let Some(parsers) = parsers {
        match &parsers.trackers {
            Some(trackers) => trackers(descs, timestamp, attributes),
            None => Err("No trackers".to_string()),
        }
    } else {
        Err("()".to_string())
    }.ok()
}

