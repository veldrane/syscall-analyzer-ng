use std::fmt::Debug;
use trackers::descriptors::Descs;
use std::any::Any;
use std::rc::Rc;

use crate::parsers::Parsable;

pub type TrackerFn = Box<dyn Fn(Rc<dyn Parsable>,&mut Descs) -> Result<Box<dyn Trackable>, String> + Send + Sync>;

// Wrapper pro tracker, který volá statickou metodu track
//pub fn tracker_wrapper<T: Trackable>(attrs: Box<dyn Any>, descs: &mut Descs) -> Result<Box<dyn Any>, String> {
//    T::track(attrs, descs).map(|tracked| Box::new(tracked) as Box<dyn Any>)
//}


pub fn tracker_wrapper<T: Trackable>(attrs: Rc<dyn Parsable>, descs: &mut Descs) -> Result<Box<dyn Trackable>, String> {
    T::track(attrs, descs).map(|tracked| Box::new(tracked) as Box<dyn Trackable>)
}

// Trait pro trackování – jeho metoda track nyní přijímá zabalený výsledek parsování

#[typetag::serde]
pub trait Trackable: 'static + Debug {
    fn track(attrs: Rc<dyn Parsable>, descs: &mut Descs) -> Result<Self, String>
    where
        Self: Sized;
}