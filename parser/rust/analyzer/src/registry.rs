use std::collections::HashMap;
use std::fmt::Debug;
use serde::{Serialize};

pub type ParserFn = Box<dyn Fn(&str) -> Result<Box<dyn SyscallArguments>, String> + Send + Sync>;

pub type Registry = HashMap<String, ParserFn>;

pub trait RegistryBuilder {
    fn new() -> Self;
}

#[typetag::serde]
pub trait SyscallArguments: 'static + Debug {
    fn parse(input: &str) -> Result<Self, String>
    where
        Self: Sized;

    fn register(registry: &mut Registry, name: &'static str)
    where
        Self: Sized,
    {
        registry.insert(
            name.to_string(),
            Box::new(|input: &str| {
                Self::parse(input)
                    .map(|parsed| Box::new(parsed) as Box<dyn SyscallArguments>)
            }),
        );
    }
}

impl RegistryBuilder for Registry {
    fn new() -> Self {
        HashMap::new()
    }
}