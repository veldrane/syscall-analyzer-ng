use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;
use serde_json::value::Value;
use registry::registry::{SyscallArguments, SyscallResults};

#[derive(Debug)]
pub struct Syscall<'a> {
    pub id: &'a i32,
    pub timestamp: &'a str,
    pub name: &'a str,
    pub args: Box<dyn SyscallArguments>,
    pub returns: Option<Box<dyn SyscallResults>>,
    pub result: &'a str,
    pub duration: &'a str,
}


impl<'a> Serialize for Syscall<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;

        map.serialize_entry("id", self.id)?;
        map.serialize_entry("timestamp", self.timestamp)?;
        map.serialize_entry("name", self.name)?;
        map.serialize_entry("result", self.result)?;
        map.serialize_entry("duration", self.duration)?;

        let args_value = serde_json::to_value(&self.args).map_err(serde::ser::Error::custom)?;
        serialize_syscall_parts(&mut map, args_value)?;
        let returns_value = serde_json::to_value(&self.returns).map_err(serde::ser::Error::custom)?;
        serialize_syscall_parts(&mut map, returns_value)?;

        map.end()
    }
}

fn serialize_syscall_parts<S>(serialize: &mut S, value: Value) -> Result<(), S::Error>
where
    S: SerializeMap,
{
    
    if let Value::Object(args_map) = value {
        for (_, value) in args_map {
            match value {
                Value::Object(s) => {
                    for (k, v) in s {
                        match &v {
                            Value::String(x) => {
                                if x == "" {
                                    continue;
                                }
                            },
                            _ => {   
                            }
                        }
                        serialize.serialize_entry(&k, &v)?;
                    }
                },
                _ => {
                    println!("Unexpected value type: {:?}", value);
                }
            }
        }
    }
    Ok(())
}