use std::fmt::Debug;
use crate::parsers::Parsable;

pub type TrackerFn = Box<dyn Fn(Box<dyn Parsable>) -> Result<Box<dyn Trackable>, String> + Send + Sync>;

// Wrapper pro tracker, který volá statickou metodu track
pub fn tracker_wrapper<T: Trackable>(parsed: Box<dyn Parsable>) -> Result<Box<dyn Trackable>, String> {
    T::track(parsed).map(|tracked| Box::new(tracked) as Box<dyn Trackable>)
}

// Trait pro trackování – jeho metoda track nyní přijímá zabalený výsledek parsování
pub trait Trackable: 'static + Debug {
    fn track(parsed: Box<dyn Parsable>) -> Result<Self, String>
    where
        Self: Sized;
}