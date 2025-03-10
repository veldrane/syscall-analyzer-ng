use std::fmt::Debug;
use std::ops::Deref;

pub type ParserArgsFn = Box<dyn Fn(&str) -> Result<Box<dyn SyscallArguments>, String> + Send + Sync>;
pub type ParserResultsFn = Box<dyn Fn(&str) -> Result<Box<dyn SyscallResults>, String> + Send + Sync>;

pub fn args_wrapper<T: SyscallArguments>(input: &str) -> Result<Box<dyn SyscallArguments>, String> {
    T::parse(input).map(|parsed| Box::new(parsed) as Box<dyn SyscallArguments>)
}


pub fn results_wrapper<T: SyscallResults>(input: &str) -> Result<Box<dyn SyscallResults>, String> {
    T::parse(input).map(|parsed| Box::new(parsed) as Box<dyn SyscallResults>)
}


pub struct  Register {
    pub arguments: ParserArgsFn,
    pub returns: Option<ParserResultsFn>,
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
 pub trait SyscallResults: 'static + Debug {
    fn parse(input: &str) -> Result<Self, String>
    where
        Self: Sized;
 }