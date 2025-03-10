use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;
use serde_json::value::Value;
use registry::registry::SyscallArguments;

#[derive(Debug)]
pub struct Syscall<'a> {
    pub id: &'a i32,
    pub timestamp: &'a str,
    pub name: &'a str,
    pub args: Box<dyn SyscallArguments>,
    pub result: &'a str,
    pub duration: &'a str,
}


impl<'a> Serialize for Syscall<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Neznáme přesný počet polí, tak zadáme None
        let mut map = serializer.serialize_map(None)?;

        map.serialize_entry("id", self.id)?;
        map.serialize_entry("timestamp", self.timestamp)?;
        map.serialize_entry("name", self.name)?;
        map.serialize_entry("result", self.result)?;
        map.serialize_entry("duration", self.duration)?;

        let args_value = serde_json::to_value(&self.args).unwrap();
        if let Value::Object(args_map) = args_value {
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
                            map.serialize_entry(&k, &v)?;
                        }
                    },
                    _ => {
                        println!("Unexpected value type: {:?}", value);
                    }
                }
            }
        }
        map.end()
    }
}
