
use std::fs::read_to_string;
use std::process::exit;
use parsers::syscall::Syscall;
use registry::registry::SyscallArguments;
use parsers::default;

use modules::init;
use regex::Regex;

use elasticsearch::{
    auth::Credentials, http::transport::{SingleNodeConnectionPool, TransportBuilder}, Elasticsearch, BulkParts 
};
use elasticsearch::http::request::JsonBody;
use url::Url;
use serde_json::{json, Value};
use tokio::runtime::Runtime;


const BASIC_SYSCALL: &str = r"(?P<timestamp>\d+.\d+)\s(?P<syscall>\w+)\((?P<arguments>.*)\)\s*\=\s(?P<result>.*)\s<(?P<duration>\d+\.\d+)>";


/* Strace parameters for the parser
strace -y -T -ttt -ff -xx -qq -o curl $CMD
*/

const STRACE_OUTPUT: &str = "../../../tests/syscalls/nginx-all.out";

fn main() {
    let re = Regex::new(BASIC_SYSCALL).unwrap();
    let registry = init::init_registry();
    let rt = Runtime::new().expect("Nelze vytvořit runtime");

    let url = Url::parse("http://10.4.4.100:9200").unwrap();
    let conn_pool = SingleNodeConnectionPool::new(url);
    let credentials = Credentials::Basic("elastic".into(), "test123".into());
    let transport = TransportBuilder::new(conn_pool).disable_proxy().auth(credentials).build().unwrap();
    let client = Elasticsearch::new(transport);

    let mut body: Vec<JsonBody<_>> = Vec::with_capacity(4000);

    let mut id = 0;
    for line in read_to_string(STRACE_OUTPUT).unwrap().lines() {
        id += 1;

        let fields = match re.captures(line) {
            Some(captures) => captures,
            None => {
                eprintln!("Řádek neodpovídá formátu: {}", line);
                continue;
            },
        };

        let result = if let Some(parser) = registry.get(&fields["syscall"]) {
            parser(fields["arguments"].as_ref())
        } else {
            default::DefaultArgs::parse(fields["arguments"].as_ref())
                .map(|v| Box::new(v) as Box<dyn SyscallArguments>)
        };

        match result {
            Ok(parsed_args) => {
                let syscall = Syscall {
                    id: id,
                    timestamp: fields["timestamp"].to_string(),
                    name: fields["syscall"].to_string(),
                    args: parsed_args,
                    result: fields["result"].to_string(),
                    duration: fields["duration"].to_string(),
                };
                match serde_json::to_value(&syscall) {
                    Ok(json) => {
                        body.push(json!({"index": {"_id": id.to_string()}}).into());
                        body.push(json.into());
                        //println!("{}", json) 
                    },
                    Err(e) => eprintln!("Chyba při serializaci syscallu {}: {}", &fields["syscall"], e),
                }
            },
            Err(e) => {
                eprintln!("Chyba při parsování syscallu {}: {}\n line: {}", &fields["syscall"], e, line);
            },
        }
    }

    println!("Odesílám data do ElasticSearch");

    rt.block_on(async {
        store_to_elastic(client, body).await
    }).unwrap(); 

    exit(0);
}


async fn store_to_elastic(client: Elasticsearch, body: Vec<JsonBody<Value>>) -> Result<(), Box<dyn std::error::Error>> {

    let response = client
    .bulk(BulkParts::Index("syscalls3"))
    .body(body)
    .send()
    .await?;

    Ok(())
}

