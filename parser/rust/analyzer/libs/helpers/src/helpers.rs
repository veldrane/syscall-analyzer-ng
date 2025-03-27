use std::str::FromStr;
use std::ops::Deref;
use serde::ser::SerializeMap;
use serde_json::value::Value;
use wrappers::parsers::Parsable;


pub struct HexString(String);


pub fn split_fd_parts(parts: &str) -> (i32, String) {

    let fd_parts: Vec<String> = parts
    .chars()
    .filter(|&c| !r#""\">?"#.contains(c))
    .collect::<String>()
    .split('<')
    .map(str::to_string)
    .collect::<Vec<String>>();

    let fd = match fd_parts[0].parse::<i32>() {
        Ok(fd) => fd,
        Err(_) => {
            // eprintln!("Error parsing fd: {}", fd_parts[0]);
            -1
        }
    };

    if fd_parts.len() < 2 {
        return (fd, "".to_string());
    }

    let file_name = match HexString::from_str(&fd_parts[1]) {
        Ok(file_name) => file_name.to_string(),
        Err(_) => {
            // eprintln!("Error parsing filename: {}", fd_parts[1]);
            "".to_string()
        }
    };

    (fd, file_name)
}

pub fn split_fd_parts_to_refs(parts: &str) -> (&str, &str) {
    let mut split_iter = parts.split('<');
    let fd = split_iter.next().expect("Missing fd part").trim();
    let filename = split_iter.next().expect("Missing filename part").trim();
    (fd, filename)
}
pub fn split_fd_parts_to_strings(parts: &str) -> (String, String) {

    let fd_parts: Vec<String> = parts
    .chars()
    .filter(|&c| !r#""\">? "#.contains(c))
    .collect::<String>()
    .split('<')
    .map(str::to_string)
    .collect::<Vec<String>>();

    let fd = fd_parts[0].parse::<String>().unwrap();
    let filename = HexString::from_str(&fd_parts[1]).unwrap().to_string();
    //let filename = HexString::fd_parts[1].clone(from_str().unwrap();
    (fd, filename)
}


impl FromStr for HexString {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes: Vec<u8> = s
            .split("x")
            .filter(|s| !s.is_empty())
            .map(|hex| u8::from_str_radix(hex, 16))
            .collect::<Result<_, _>>()?;
        Ok(HexString(String::from_utf8_lossy(&bytes).to_string()))
    }
}


impl Deref for HexString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


pub fn flat_serializer<S>(serialize: &mut S, parsable_value: &Box<dyn Parsable>) -> Result<(), S::Error>
where
    S: SerializeMap,
{
    
    let value = serde_json::to_value(&parsable_value).map_err(serde::ser::Error::custom)?;

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