use std::fmt::Debug;

pub type ParserFn = Box<dyn Fn(&str, Option<&str>) -> Result<Box<dyn Parsable>, String> + Send + Sync>;

pub fn parser_wrapper<T: Parsable>(args: &str, results: Option<&str>) -> Result<Box<dyn Parsable>, String> {
    T::parse(args, results).map(|parsed| Box::new(parsed) as Box<dyn Parsable>)
}

#[typetag::serde]
 pub trait Parsable: 'static + Debug {
    fn parse(args: &str, results: Option<&str>) -> Result<Self, String>
    where
        Self: Sized;
 }
