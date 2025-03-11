use std::fmt::Debug;
use std::ops::Deref;

pub type ParserFn = Box<dyn Fn(&str) -> Result<Box<dyn Parsable>, String> + Send + Sync>;


pub fn parser_wrapper<T: Parsable>(input: &str) -> Result<Box<dyn Parsable>, String> {
    T::parse(input).map(|parsed| Box::new(parsed) as Box<dyn Parsable>)
}



pub struct  Register {
    pub arguments: ParserFn,
    pub returns: Option<ParserFn>,
}


impl Deref for Register {

    type Target = Register;

    fn deref(&self) -> &Self::Target {
        &self
    }
}

#[typetag::serde]
pub trait Parsable: 'static + Debug {
    fn parse(input: &str) -> Result<Self, String>
    where
        Self: Sized;
 }
