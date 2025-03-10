// We prefer to keep `main.rs` and `lib.rs` separate as it makes it easier to add extra helper
// binaries later which share code with the main project. It could save you from a nontrivial
// refactoring effort in the future.
//
// Whether to make `main.rs` just a thin shim that awaits a `run()` function in `lib.rs`, or
// to put the application bootstrap logic here is an open question. Both approaches have their
// upsides and their downsides. Your input is welcome!

use anyhow::Context;
use url::Url;
use clap::Parser;
use analyzer_be::config::Config; // Add this line to import the Config type
use analyzer_be::http;

use elasticsearch::{
    auth::Credentials, http::transport::{SingleNodeConnectionPool, TransportBuilder}, Elasticsearch, cat::CatIndicesParts
};

use serde_json::Value;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // This returns an error if the `.env` file doesn't exist, but that's not what we want
    // since we're not going to use a `.env` file if we deploy this application.
    dotenv::dotenv().ok();

    // Initialize the logger.
    env_logger::init();

    // Parse our configuration from the environment.
    // This will exit with a help message if something is wrong.
    let config = Config::parse();

    let elastic = get_elastic_connection(&config).await.context("could not connect to elastic")?;

    // println!("{:?}", response_body);


    // We create a single connection pool for SQLx that's shared across the whole application.
    // This saves us from opening a new connection for every API call, which is wasteful.

    // Finally, we spin up our API.
    http::serve(config, elastic).await?;

    Ok(())
}

async fn get_elastic_connection(config: &Config) -> anyhow::Result<Elasticsearch> {
    
    let url = Url::parse(&config.elastic_url)?;
    let client = Elasticsearch::new(TransportBuilder::new(SingleNodeConnectionPool::new(url))
        .auth(Credentials::
            Basic(config.username.to_string(), config.password.to_string()))
        .build()?);

    // Test connect to elastic, if failes, return error
    let response = client
    .cat()
    .indices(CatIndicesParts::Index(&["*"]))
    .format("json")
    .send()
    .await?;

    response.json::<Value>().await?;

    Ok(client)
}
