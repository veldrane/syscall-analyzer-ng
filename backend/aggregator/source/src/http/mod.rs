use crate::config::Config;
use anyhow::Context;
use axum::{extract::Extension, routing::get, Json, Router};
use elasticsearch::Elasticsearch;
use std::sync::Arc;
use tower::ServiceBuilder;
use std::net::SocketAddr;
use serde_json::json;

// Utility modules.

/// Defines a common error type to use for all request handlers, compliant with the Realworld spec.
mod error;



/// A catch-all module for other common types in the API. Arguably, the `error` and `extractor`
/// modules could have been children of this one, but that's more of a subjective decision.
mod types;



pub use error::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

use tower_http::trace::TraceLayer;

/// The core type through which handler functions can access common API state.
///
/// This can be accessed by adding a parameter `Extension<ApiContext>` to a handler function's
/// parameters.
///
/// In other projects I've passed this stuff as separate objects, e.g.
/// using a separate actix-web `Data` extractor for each of `Config`, `PgPool`, etc.
/// It just ends up being kind of annoying that way, but does have the whole
/// "pass only what you need where you need it" angle.
///
/// It may not be a bad idea if you need your API to be more modular (turn routes
/// on and off, and disable any unused extension objects) but it's really up to a
/// judgement call.
#[derive(Clone)]
struct ApiContext {
    config: Arc<Config>,
    elastic: Elasticsearch,
}

pub async fn serve(config: Config, elastic: Elasticsearch) -> anyhow::Result<()> {
    // Bootstrapping an API is both more intuitive with Axum than Actix-web but also
    // a bit more confusing at the same time.
    //
    // Coming from Actix-web, I would expect to pass the router into `ServiceBuilder` and not
    // the other way around.
    //
    // It does look nicer than the mess of `move || {}` closures you have to do with Actix-web,
    // which, I suspect, largely has to do with how it manages its own worker threads instead of
    // letting Tokio do it.
    let app = api_router().layer(
        ServiceBuilder::new()
            // The other reason for using a single object is because `AddExtensionLayer::new()` is
            // rather verbose compared to Actix-web's `Data::new()`.
            //
            // It seems very logically named, but that makes it a bit annoying to type over and over.
            .layer(Extension(ApiContext {
                config: Arc::new(config),
                elastic,
            }))
            // Enables logging. Use `RUST_LOG=tower_http=debug`
            .layer(TraceLayer::new_for_http()),
    );

    // We use 8080 as our default HTTP server port, it's pretty easy to remember.
    //
    // Note that any port below 1024 needs superuser privileges to bind on Linux,
    // so 80 isn't usually used as a default for that reason.

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .context("error running HTTP server")
}

fn api_router() -> Router {
    // This is the order that the modules were authored in.
    //users::router()
      //  .merge(profiles::router())
        //.merge(articles::router())
    return  Router::new().route("/",get(|| async {
        let value = json!({
            "status": "Server is UP!"
        });

        Json(value)
     }));
}