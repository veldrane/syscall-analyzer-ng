use std::fmt::Debug;
use trackers::fd_table::Descs;
use std::rc::Rc;

use crate::parsers::Parsable;

pub type TrackerFn = Box<dyn Fn(&mut Descs, f64, Rc<dyn Parsable>) -> Result<Box<dyn Trackable>, String> + Send + Sync>;

// Wrapper pro tracker, který volá statickou metodu track
//pub fn tracker_wrapper<T: Trackable>(attrs: Box<dyn Any>, descs: &mut Descs) -> Result<Box<dyn Any>, String> {
//    T::track(attrs, descs).map(|tracked| Box::new(tracked) as Box<dyn Any>)
//}


pub fn as_dyn_tracker<T: Trackable>(descs: &mut Descs, timestamp: f64, attrs: Rc<dyn Parsable>) -> Result<Box<dyn Trackable>, String> {
    T::track(descs, timestamp, attrs ).map(|tracked| Box::new(tracked) as Box<dyn Trackable>)
}

// Trait pro trackování – jeho metoda track nyní přijímá zabalený výsledek parsování

#[typetag::serde]
pub trait Trackable: 'static + Debug {
    fn track(descs: &mut Descs, timestamp: f64, attrs: Rc<dyn Parsable>) -> Result<Self, String>
    where
        Self: Sized;
}