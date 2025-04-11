use std::{collections::HashMap, process::exit, sync::mpsc};
use registry::registry::RegistryEntry;
use trackers::{fd_table::Descs, archive::Archive};
use modules::{ logging::Logger as l, registrator as r, inputs, processors as p};
use regex::Regex;



const BASIC_SYSCALL: &str = r"(?P<timestamp>\d+.\d+)\s(?P<name>\w+)\((?P<arguments>.*)\)\s*\=\s(?P<result>.*)\s<(?P<duration>\d+\.\d+)>";


/* Strace parameters for the parser
strace -y -T -ttt -ff -xx -qq -o curl $CMD
*/

const STRACE_DIR: &str = "/home/veldrane/Bitbucket/private/syscall-analyzer-ng/tests/sshd";


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let registry = r::build();
    let log = l::build();

    log.info("Syscall analyzer started...".to_string());
    run(&registry, &log)?;
    Ok(())
}


fn run(registry: &HashMap<String, RegistryEntry>, log: &l) -> Result<(), Box<dyn std::error::Error>> {

    let basic_regex = Regex::new(BASIC_SYSCALL)?;
    let (pid, init_timestamp) = inputs::find_first(STRACE_DIR).expect("First trace file not found");
    let mut archive = Archive::new();

    archive.add_descs(pid, Descs::with_std_fds(init_timestamp));

    let mut worklist: Vec<i32> = [pid].to_vec();
    let mut id_counter = 0;
    
    while let Some(pid) = worklist.pop() {
        log.info(format!("Process the trace for new pid {}", pid));
        id_counter = p::process_pid(
            &STRACE_DIR,
            pid,
            init_timestamp,
            &basic_regex,
            registry,
            &mut archive,
            id_counter,
            &mut worklist,
            &log
        )?;
    }
    exit(0);
}


