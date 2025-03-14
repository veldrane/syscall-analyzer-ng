use std::fs::read_to_string;
use std::process::exit;
use parsers::syscall::Syscall;
use registry::registry::{Parsable, Register};
use parsers::default;
use std::collections::HashMap;
use std::time::SystemTime;

use modules::init;
use regex::Regex;
//use once_cell::sync::Lazy;
use once_cell::sync::Lazy;

use elasticsearch::{
    auth::Credentials, http::transport::{SingleNodeConnectionPool, TransportBuilder}, Elasticsearch, BulkParts 
};
use elasticsearch::http::request::JsonBody;
use url::Url;
use serde_json::{json, Value};
use tokio::runtime::Runtime;


const BASIC_SYSCALL: &str = r"(?P<timestamp>\d+.\d+)\s(?P<syscall>\w+)\((?P<arguments>.*)\)\s*\=\s(?P<result>.*)\s<(?P<duration>\d+\.\d+)>";
//const BASIC_SYSCALL: &str = r"(?P<timestamp>\d+.\d+)\s(?P<syscall>\w+)\((?P<arguments>.*)\)\s*\=\s(?:(?P<result>.*?)\s<(?P<duration>\d+\.\d+)>|\?)";

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(BASIC_SYSCALL).unwrap());

/* Strace parameters for the parser
strace -y -T -ttt -ff -xx -qq -o curl $CMD
*/

//const STRACE_OUTPUT: &str = "../../../tests/syscalls/nginx-all.out";
const STRACE_OUTPUT: &str = "../../../tests/libreoffice/libreoffice.out";


fn main() -> Result<(), Box<dyn std::error::Error>> {


    //let registry = Register::new();

    let registry = init::init_registry();

    run(&registry)?;
    Ok(())
}


fn run(registry: &HashMap<String, Register>) -> Result<(), Box<dyn std::error::Error>> {

    eprintln!("{}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis());

    //let rt = Runtime::new().expect("Nelze vytvořit runtime");

    //let url = Url::parse("http://10.4.4.100:9200").unwrap();
    //let conn_pool = SingleNodeConnectionPool::new(url);
    //let credentials = Credentials::Basic("elastic".into(), "test123".into());
    //let transport = TransportBuilder::new(conn_pool).disable_proxy().auth(credentials).build().unwrap();
    //let client = Elasticsearch::new(transport);

    //let mut body: Vec<JsonBody<_>> = Vec::with_capacity(4000);

    let mut id = 0;
    for line in read_to_string(STRACE_OUTPUT)?.lines() {

        id += 1;
        let fields = match RE.captures(line) {
            Some(captures) => captures,
            None => {
                eprintln!("Řádek neodpovídá formátu: {}", line);
                continue;
            },
        };

        let parsers = registry.get(&fields["syscall"]);

        let parsed_arguments = if let Some(parsers) = parsers {
            (parsers.arguments)(fields["arguments"].as_ref())
        } else {
            default::DefaultArgs::parse(fields["arguments"].as_ref())
                .map(|v| Box::new(v) as Box<dyn Parsable>)
        };

        let arguments = match parsed_arguments {
            Ok(parsed_args) => parsed_args,
            Err(e) => {
                eprintln!("Chyba při parsování syscallu {}: {}\n line: {}", &fields["syscall"], e, line);
                continue;
            },
        };

        let results = if let Some(parsers) = parsers {
            match parsers.results.as_ref().map(|f| f(fields["result"].as_ref())).transpose() {
                Ok(value) => value,
                Err(_) => None,
            }
        } else {
            None
        };
        
        

        // registry.get(&fields["syscall"]);   PROC ?

        let syscall = Syscall {
            id: &id,
            timestamp: fields["timestamp"].as_ref(),
            name: fields["syscall"].as_ref(),
            args: arguments,
            results: results,
            result: fields["result"].as_ref(),
            duration: fields["duration"].as_ref(),
        };

        println!("{}", serde_json::to_string(&syscall).unwrap());
        //println!("{:?}", syscall);
    }

    //println!("Odesílám data do ElasticSearch");

    //rt.block_on(async {
    //    store_to_elastic(client, body).await
    //}).unwrap(); 

    eprintln!("{}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis());

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

