use std::fmt::Debug;
use std::ops::Deref;

pub type ParserArgsFn = Box<dyn Fn(&str) -> Result<Box<dyn SyscallArguments>, String> + Send + Sync>;
pub type ParserReturnsFn = Box<dyn Fn(&str) -> Result<Box<dyn SyscallReturns>, String> + Send + Sync>;

pub fn args_wrapper<T: SyscallArguments>(input: &str) -> Result<Box<dyn SyscallArguments>, String> {
    T::parse(input).map(|parsed| Box::new(parsed) as Box<dyn SyscallArguments>)
}


pub fn returns_wrapper<T: SyscallReturns>(input: &str) -> Result<Box<dyn SyscallReturns>, String> {
    T::parse(input).map(|parsed| Box::new(parsed) as Box<dyn SyscallReturns>)
}


pub struct  Register {
    pub arguments: ParserArgsFn,
    pub returns: Option<ParserReturnsFn>,
}


impl Deref for Register {

    type Target = Register;

    fn deref(&self) -> &Self::Target {
        &self
    }
}

#[typetag::serde]
pub trait SyscallArguments: 'static + Debug {
    fn parse(input: &str) -> Result<Self, String>
    where
        Self: Sized;
 }

 #[typetag::serde]
 pub trait SyscallReturns: 'static + Debug {
    fn parse(input: &str) -> Result<Self, String>
    where
        Self: Sized;
 }