use std::collections::HashMap;
use regex::Regex;
use serde::{Serialize, Serializer};
use std::result::Result;


const BASIC_SYSCALL: &str = r"(?P<timestamp>\d+.\d+)\s(?P<syscall>\w+)\((?P<arguments>.*)\)\s+\=\s(?P<results>.*<(?P<duration>\d+\.\d+)>)";

pub trait Argumetable {
    fn get_arguments(&self, syscall: &mut Syscall) -> Result<(), ArgumentsError>;
}

#[derive(Debug, Clone)]
pub enum SyscallKey {
    Int(i32),
    Float(f64),
    Str(String),
}

#[derive(Debug, Clone, Serialize)]
pub enum Syscalls {
    General,
    Openat,
    Mmap
}

#[derive(Debug, Clone)]
pub enum ArgumentsError {
    NotParsable,
    InvalidArgument,
}

pub struct General {
    pub timestamp: String,
    pub syscall: String,
    pub arguments: String,
    pub results: String,
    pub duration: f64,
}

impl General {

    pub fn new(row: &str) -> Option<General> {
        let re = Regex::new(BASIC_SYSCALL).unwrap();
        
        match re.captures(row) {
            Some(row) => {
                Some(General {
                    timestamp: row["timestamp"].to_string(),
                    syscall: row["syscall"].to_string(),
                    arguments: row["arguments"].to_string(),
                    results: row["results"].to_string(),
                    duration: row["duration"].parse().unwrap(),
                })
            },
            None => None
        }
    }
    
}


impl Argumetable for General {
    fn get_arguments(&self, syscall: &mut Syscall) -> Result<(), ArgumentsError> {
        syscall.insert("timestamp".to_string(), SyscallKey::Str(self.timestamp.clone()));
        syscall.insert("syscall".to_string(), SyscallKey::Str(self.syscall.clone()));
        syscall.insert("arguments".to_string(), SyscallKey::Str(self.arguments.clone()));
        syscall.insert("results".to_string(), SyscallKey::Str(self.results.clone()));
        syscall.insert("durations".to_string(), SyscallKey::Float(self.duration));
        Result::Ok(())
    }
}

impl Serialize for SyscallKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SyscallKey::Int(val) => serializer.serialize_i32(*val),
            SyscallKey::Float(val) => serializer.serialize_f64(*val),
            SyscallKey::Str(val) => serializer.serialize_str(val),
        }
    }
}

#[derive(Debug,Serialize,Clone)]
pub struct Syscall(pub HashMap<String, SyscallKey>);


impl Syscall {
    pub fn new(row :&str) -> Syscall {
        let re = Regex::new(BASIC_SYSCALL).unwrap();
        let mut syscall:    HashMap<String, SyscallKey> = HashMap::new();
        let row = re.captures(row).unwrap();
        
        syscall.insert("timestamp".to_string(), SyscallKey::Str(row["timestamp"].to_string()));
        syscall.insert("syscall".to_string(), SyscallKey::Str(row["syscall"].to_string()));
        syscall.insert("arguments".to_string(), SyscallKey::Str(row["arguments"].to_string()));
        syscall.insert("results".to_string(), SyscallKey::Str(row["results"].to_string()));
        syscall.insert("duration".to_string(), SyscallKey::Float(row["duration"].parse().unwrap()));

        return Syscall(syscall);

    }

    pub fn get(&self, key: &str) -> Option<&SyscallKey> {
        self.0.get(key)
    }

    pub fn insert(&mut self, key: String, value: SyscallKey) {
        self.0.insert(key, value);
    }
}