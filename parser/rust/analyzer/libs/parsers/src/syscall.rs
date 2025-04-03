use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;
use helpers::helpers::generic_serializer;
use wrappers::parsers::Parsable;
use wrappers::trackers::Trackable;
use std::rc::Rc;


#[derive(Debug)]
pub struct Syscall<'a> {
    pub id: &'a i32,
    pub timestamp: &'a f64,
    pub name: &'a str,
    pub attributes: Rc<dyn Parsable>,
    pub trackers: Option<Box<dyn Trackable>>,
    pub result: &'a str,
    pub duration: &'a str,
}



impl<'a> Serialize for Syscall<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;

        let attributes = Rc::clone(&self.attributes);

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