use std::collections::HashMap;

use helpers::helpers::split_fd_parts;
use registry::registry::Parsable;
use regex::Regex;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::value::Value;
use indexmap::IndexMap;

const SENDMSG_SYSCALL_ARGS: &str = r"(?P<socket_raw>.+)\,\s\{(?P<msg_args>.*)\}\,\s(?P<flags>.+)";
const MSG_ARGS: &str = r"msg_name\=(?P<msg_name>.+)\,\smsg_namelen\=(?P<msg_namelen>.+)\,\smsg_iov\=\[\{(?P<msg_iov>.*)\}\]\,\smsg_iovlen\=(?P<msg_iovlen>.*)\,\smsg_control\=\[\{(?P<msg_control>.*)\}\]\,\smsg_controllen\=(?P<msg_controllen>.*)\,\smsg_flags\=(?P<msg_flags>.*)";

#[derive(Debug, Deserialize)]
enum MsgArgsOutput {
    Raw(String),
    Parsed(MsgArgs),
    None
}

impl Default for MsgArgsOutput {
    fn default() -> Self {
        MsgArgsOutput::None
    }
}

// const ACCEPT_SYSCALL_ARGS: &str = r"(?P<socket_raw>.*)\,\s*\{(?P<sock_addr>.*)\}\,\s(?P<sock_len>.*)";

#[derive(Debug, Deserialize, Default)]
pub struct SendmsgArgs {
    socket_fd: i32,
    socket_name: String,
    msg_args: MsgArgsOutput,
    flags: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MsgArgs {
    msg_name: String,
    msg_namelen: String,
    msg_iov: MsgIov,
    msg_iovlen: String,
    msg_control: MsgControl,
    msg_controllen: String,
    msg_flags: String
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MsgIov {
    iov_base: String,
    iov_len: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MsgControl {
    cmsg_len: String,
    cmsg_level: String,
    cmsg_type: String,
    cmsg_data: String,
    cmsg_fd: String,
    cmsg_file_name: String,
}


#[typetag::serde]
impl Parsable for SendmsgArgs {
    fn parse(input: &str) -> Result<Self, String> {
        

        let mut arguments = SendmsgArgs::default();
        // let mut flags= 0;
        let re = Regex::new(SENDMSG_SYSCALL_ARGS).unwrap();
        let caps = re.captures(&input).unwrap();
        (arguments.socket_fd, arguments.socket_name) = split_fd_parts(&caps["socket_raw"]);


        let re_msg = Regex::new(MSG_ARGS).unwrap();
        let msg_caps = match re_msg.captures(&caps["msg_args"]) {
            Some(caps) => caps,
            None => {
                arguments.msg_args = MsgArgsOutput::Raw(caps["msg_args"].to_string());
                return Ok(arguments);
            }
        };

        let msg_iov_parts: HashMap<String, String> = msg_caps["msg_iov"].to_string()
                                    .chars()
                                    .filter(|&c| !r#""\"? "#.contains(c))
                                    .collect::<String>()
                                    .split(',')
                                    .filter_map(|pair| {
                                        let mut attr = pair.split('=');
                                        Some((attr.next()?.to_string(), attr.next()?.to_string()))
                                    })
                                    .collect();
        
        let iov_len = &msg_iov_parts["iov_len"];
        let iov_base = &msg_iov_parts["iov_base"];

        
        let cmsg_parts: HashMap<String, String> = msg_caps["msg_control"].to_string()
                                    .chars()
                                    .filter(|&c| !r#""\"? "#.contains(c))
                                    .collect::<String>()
                                    .split(',')
                                    .filter_map(|pair| {
                                        let mut attr = pair.split('=');
                                        Some((attr.next()?.to_string(), attr.next()?.to_string()))
                                    })
                                    .collect();

        let (cmsg_fd, cmsg_file_name) = if cmsg_parts["cmsg_type"] == "SCM_RIGHTS" {
            let clean_cmsg_data = cmsg_parts["cmsg_data"]
                .chars().filter(|&c| !r#""\"\[\]? "#
                .contains(c))
                .collect::<String>();
            
            split_fd_parts(&clean_cmsg_data)
        } else {
            (0_i32, "".to_string())
        };

        arguments.msg_args = MsgArgsOutput::Parsed(MsgArgs{
            msg_name: msg_caps["msg_name"].to_string(),
            msg_namelen: msg_caps["msg_namelen"].to_string(),
            msg_iov: MsgIov{
                iov_base: iov_base.to_string(),
                iov_len: iov_len.to_string()
            },
            msg_iovlen: msg_caps["msg_iovlen"].to_string(),
            msg_control: MsgControl{
                cmsg_len: cmsg_parts["cmsg_len"].to_string(),
                cmsg_level: cmsg_parts["cmsg_level"].to_string(),
                cmsg_type: cmsg_parts["cmsg_type"].to_string(),
                cmsg_data: cmsg_parts["cmsg_data"].to_string(),
                cmsg_fd: cmsg_fd.to_string(),
                cmsg_file_name: cmsg_file_name.to_string()
            },
            msg_controllen: msg_caps["msg_controllen"].to_string(),
            msg_flags: msg_caps["msg_flags"].to_string()
        });

        Ok(arguments)
    }   
}

impl Serialize for SendmsgArgs {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Vytvoříme top-level mapu
        let mut map:IndexMap<String, Value> = IndexMap::new();

        map.insert("socket_fd".to_string(), Value::Number(self.socket_fd.into()));
        map.insert("socket_name".to_string(), Value::String(self.socket_name.clone()));
        map.insert("flags".to_string(), Value::String(self.flags.clone()));

        match &self.msg_args {
            MsgArgsOutput::Parsed(ref msg_args) => {
                map.insert("msg_name".to_string(), Value::String(msg_args.msg_name.clone()));
                map.insert("msg_namelen".to_string(), Value::String(msg_args.msg_namelen.clone()));
                map.insert("msg_iovlen".to_string(), Value::String(msg_args.msg_iovlen.clone()));
                map.insert("msg_controllen".to_string(), Value::String(msg_args.msg_controllen.clone()));
                map.insert("iov_base".to_string(), Value::String(msg_args.msg_iov.iov_base.clone()));
                map.insert("iov_len".to_string(), Value::String(msg_args.msg_iov.iov_len.clone()));
                map.insert("cmsg_len".to_string(), Value::String(msg_args.msg_control.cmsg_len.clone()));
                map.insert("cmsg_level".to_string(), Value::String(msg_args.msg_control.cmsg_level.clone()));
                map.insert("cmsg_type".to_string(), Value::String(msg_args.msg_control.cmsg_type.clone()));
                map.insert("cmsg_data".to_string(), Value::String(msg_args.msg_control.cmsg_data.clone()));
                map.insert("cmsg_fd".to_string(), Value::String(msg_args.msg_control.cmsg_fd.clone()));
                map.insert("cmsg_file_name".to_string(), Value::String(msg_args.msg_control.cmsg_file_name.clone()));
            },
            MsgArgsOutput::Raw(msg_args) => {
                map.insert("msg_args".to_string(), Value::String(msg_args.clone()));
            },
            MsgArgsOutput::None => {}
        }
        // Serializace výsledné mapy
        map.serialize(serializer)
    }
}


