use sec_lib;

use sec_lib::tickers::{get_tickers, parse_tickers};

use lambda_runtime::{service_fn, LambdaEvent, Error as LambdaError};
use log::LevelFilter;
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    simple_logger::SimpleLogger::new().with_utc_timestamps()
        .with_level(LevelFilter::Info).init()?;
    let func = service_fn(handler);
    log::info!("In main");
    lambda_runtime::run(func).await?;
    log::info!("Leaving main");
    Ok(())
}

async fn handler(event: LambdaEvent<Value>) -> Result<Value, LambdaError> {
    log::info!("Received event.");
    let (event, _context) = event.into_parts();
    let first_name = event["firstName"].as_str().unwrap_or("world");
    log::info!("Start.");
    let response = match sec_lib::company_names::request_lookup_data().await {
        Ok(str) => str,
        Err(e) => panic!("Failed to retrieve document: {:?}", e)
    };
    log::info!("Got response.");
    let map = sec_lib::company_names::consume_response(response).await;
    sec_lib::resources::put_company_names(map).await.unwrap();
    log::info!("Finish.");
    Ok(json!({ "message": format!("Hello, {}!", first_name) }))
}
