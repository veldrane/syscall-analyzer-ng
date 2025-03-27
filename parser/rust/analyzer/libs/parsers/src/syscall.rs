use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;
use helpers::helpers::flat_serializer;
use wrappers::parsers::Parsable;
use wrappers::trackers::Trackable;


#[derive(Debug)]
pub struct Syscall<'a> {
    pub id: &'a i32,
    pub timestamp: &'a str,
    pub name: &'a str,
    pub attributes: Box<dyn Parsable>,
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

        map.serialize_entry("id", self.id)?;
        map.serialize_entry("timestamp", self.timestamp)?;
        map.serialize_entry("name", self.name)?;
        map.serialize_entry("result", self.result)?;
        map.serialize_entry("duration", self.duration)?;

        flat_serializer(&mut map, &self.attributes)?;

        //if let Some(ref results) = self.results {
        //    flat_serializer(&mut map, results)?;
        //}

        map.end()

    }
}