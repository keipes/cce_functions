use sec_lib;

use crate::sec_lib::tickers::{get_tickers, parse_tickers};

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
    log::info!("Created http client.");
    let eventually_tickers = get_tickers();
    log::info!("Started ticker request.");

    log::info!("Got S3 client.");
    let response = match eventually_tickers.await {
        Ok(r) => r,
        Err(error) => panic!("Error downloading tickers: {:?}", error)
    };
    log::info!("Received ticker response.");
    let ticker_map = parse_tickers(response).await;
    log::info!("Got ticker map.");
    let _ = match sec_lib::resources::put_tickers(ticker_map).await {
        Ok(r) => r,
        Err(error) => panic!("Put failure: {:?}", error)
    };
    Ok(json!({ "message": format!("Hello, {}!", first_name) }))
}
