use std::{any::Any, fmt::Debug};
use std::rc::Rc;

pub type ParserFn = Rc<dyn Fn(&str, Option<&str>) -> Result<Rc<dyn Parsable>, String> + Send + Sync>;

pub fn as_dyn_parser<T: Parsable>(args: &str, results: Option<&str>) -> Result<Rc<dyn Parsable>, String> {
    T::parse(args, results).map(|parsed| Rc::new(parsed) as Rc<dyn Parsable>)
}


#[typetag::serde]
 pub trait Parsable: Any + 'static + Debug {
    fn parse(args: &str, results: Option<&str>) -> Result<Self, String>
    where
        Self: Sized + typetag::Serialize + typetag::Deserialize;

    fn as_any(self: Rc<Self>) -> Rc<dyn Any>;
 }

