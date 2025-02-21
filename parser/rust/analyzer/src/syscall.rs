use serde::{Serialize, Serializer};
use serde_json::value::Value;
use indexmap::IndexMap;

use crate::registry::SyscallArguments;

#[derive(Debug)]
pub struct Syscall {
    pub timestamp: String,
    pub name: String,
    pub args: Box<dyn SyscallArguments>,
}


impl Serialize for Syscall {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Vytvoříme top-level mapu
        let mut map:IndexMap<String, Value> = IndexMap::new();

        map.insert("timestamp".to_string(), Value::String(self.timestamp.clone()));
        map.insert("name".to_string(), Value::String(self.name.clone()));
        // Serializace vnořeného typu do Value

        let args_value = serde_json::to_value(&self.args).unwrap();
        if let Value::Object(args_map) = args_value {
            for (_, value) in args_map.clone() {
                match value {
                    Value::Object(s) => {
                        for (k, v) in s {
                            map.insert(k, v);
                        }
                    },
                    _ => {
                        println!("Unexpected value type: {:?}", value);
                    }
                }
            }
        }

        // Serializace výsledné mapy
        map.serialize(serializer)
    }
}
