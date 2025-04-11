use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;
use helpers::helpers::generic_serializer;
use wrappers::{ parsers::Parsable, trackers::Trackable};
use std::rc::Rc;


use crate::default::RawAttrs;


#[derive(Debug)]
pub struct Syscall<'a> {
    pub pid: &'a i32,
    pub id: &'a i32,
    pub timestamp: &'a f64,
    pub name: &'a str,
    pub attributes: Rc<dyn Parsable>,
    pub trackers: Option<Box<dyn Trackable>>,
    pub result: &'a str,
    pub duration: &'a f64,
}



impl<'a> Serialize for Syscall<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;

        let attributes = Rc::clone(&self.attributes);

        map.serialize_entry("pid", self.pid)?;
        map.serialize_entry("id", self.id)?;
        map.serialize_entry("timestamp", self.timestamp)?;
        map.serialize_entry("name", self.name)?;
        map.serialize_entry("result", self.result)?;
        map.serialize_entry("duration", self.duration)?;

        generic_serializer(&mut map, attributes)?;

        if let Some(ref trackers) = self.trackers {
            generic_serializer(&mut map, trackers)?;
        }

        map.end()

    }
}

impl<'a> Syscall<'a> {


    pub fn new() -> Self {
        Syscall {
            pid: &0,
            id: &0,
            timestamp: &0.0,
            name: "",
            attributes: Rc::new(RawAttrs {
                raw: String::new(),
            }),
            trackers: None,
            result: "",
            duration: &0.0,
        }
    }

    pub fn with_pid(mut self, pid: &'a i32) -> Self {
        self.pid = &pid;
        self
    }

    pub fn with_id(mut self, id:&'a i32) -> Self {
        self.id = &id;
        self
    }

    pub fn with_timestamp(mut self, timestamp:&'a f64) -> Self {
        self.timestamp = &timestamp;
        self
    }

    pub fn with_name(mut self, name: &'a str) -> Self {
        self.name = name;
        self
    }
    pub fn with_attributes(mut self, attributes: Rc<dyn Parsable>) -> Self {
        self.attributes = attributes;
        self
    }
    pub fn with_trackers(mut self, trackers: Option<Box<dyn Trackable>>) -> Self {
        self.trackers = trackers;
        self
    }   

    pub fn with_result(mut self, result: &'a str) -> Self {
        self.result = result;
        self
    }
    pub fn with_duration(mut self, duration: &'a f64) -> Self {
        self.duration = duration;
        self
    }

    pub fn build(self) -> Syscall<'a> {
        Syscall {
            pid: self.pid,
            id: self.id,
            timestamp: self.timestamp,
            name: self.name,
            attributes: self.attributes,
            trackers: self.trackers,
            result: self.result,
            duration: self.duration,
        }
    }
}