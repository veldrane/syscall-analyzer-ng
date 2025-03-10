
use std::fs::read_to_string;
use std::process::exit;
use parsers::syscall::Syscall;
use registry::registry::{SyscallArguments, Register};
use parsers::default;
use std::collections::HashMap;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {


    //let registry = Register::new();

    let registry = init::init_registry();

    run(&registry)?;
    Ok(())
}


fn run(registry: &HashMap<String, Register>) -> Result<(), Box<dyn std::error::Error>> {


    let re = Regex::new(BASIC_SYSCALL)?;

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
        let fields = match re.captures(line) {
            Some(captures) => captures,
            None => {
                eprintln!("Řádek neodpovídá formátu: {}", line);
                continue;
            },
        };

        //let result = if let Some(parser) = registry.get(&fields["syscall"]) {
        //    parser(fields["arguments"].as_ref())
        //} else {
        //    default::DefaultArgs::parse(fields["arguments"].as_ref())
        //        .map(|v| Box::new(v) as Box<dyn SyscallArguments>)
        //};

        //let parsers = registry.get(&fields["syscall"]);

        let parsers = registry.get(&fields["syscall"]);

        let parsed_arguments = if let Some(parsers) = parsers {
            (parsers.arguments)(fields["arguments"].as_ref())
        } else {
            default::DefaultArgs::parse(fields["arguments"].as_ref())
                .map(|v| Box::new(v) as Box<dyn SyscallArguments>)
        };

        let parsed_returns = if let Some(parsers) = parsers {
            parsers.returns.as_ref().map(|f| f(fields["result"].as_ref())).transpose()?
        } else {
            None
        };
        
        //let parsed_arguments = if let Some(parser) = registry.get(&fields["syscall"]) {
        //    (parser.arguments)(fields["arguments"].as_ref())
        //} else {
        //    default::DefaultArgs::parse(fields["arguments"].as_ref())
        //        .map(|v| Box::new(v) as Box<dyn SyscallArguments>)
        //};

        //let returns = match parsers {
        //    Some(parser) => parser(fields["returns"].as_ref()),
         //   None => None,
        //};

        let arguments = match parsed_arguments {
            Ok(parsed_args) => parsed_args,
            Err(e) => {
                eprintln!("Chyba při parsování syscallu {}: {}\n line: {}", &fields["syscall"], e, line);
                continue;
            },
        };

        let returns = match parsed_returns {
            Some(parsed_returns) => Some(parsed_returns),
            None => None,
        };

        registry.get(&fields["syscall"]);

        let syscall = Syscall {
            id: &id,
            timestamp: fields["timestamp"].as_ref(),
            name: fields["syscall"].as_ref(),
            args: arguments,
            returns: returns,
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

