use std::ops::Deref;
use wrappers::parsers::ParserFn;


pub struct  Register {
    pub attributes: ParserFn,
}


impl Deref for Register {

    type Target = Register;

    fn deref(&self) -> &Self::Target {
        &self
    }
}



 