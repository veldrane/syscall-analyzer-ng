use std::ops::Deref;
use wrappers::parsers::ParserFn;
use wrappers::trackers::TrackerFn;


pub struct  Register {
    pub attributes: ParserFn,
    pub trackers: Option<TrackerFn>,
}


impl Deref for Register {

    type Target = Register;

    fn deref(&self) -> &Self::Target {
        &self
    }
}



 