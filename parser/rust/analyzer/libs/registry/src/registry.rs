use std::ops::Deref;
use wrappers::parsers::ParserFn;
use wrappers::trackers::TrackerFn;


pub struct  RegistryEntry {
    pub attributes: ParserFn,
    pub trackers: Option<TrackerFn>,
}


impl Deref for RegistryEntry {

    type Target = RegistryEntry;

    fn deref(&self) -> &Self::Target {
        &self
    }
}



 