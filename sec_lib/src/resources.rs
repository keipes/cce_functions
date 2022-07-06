use std::collections::HashMap;
use std::error::Error;
use aws_sdk_s3::Client;
use aws_sdk_s3::types::ByteStream;
use aws_config::meta::region::RegionProviderChain;

const BUCKET: &str = "cce-resources";
const TICKERS_FILE: &str = "tickers.json";
const COMPANIES_FILE: &str = "company_names.json";

pub async fn client() -> Result<Client, Box<dyn Error>> {
    let region_provider = RegionProviderChain::default_provider()
        .or_else("us-west-2");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);
    Ok(client)
}

pub async fn get_tickers() -> ByteStream {
    get_object(BUCKET, TICKERS_FILE).await
}

pub async fn put_tickers(tickers: HashMap<u64, Vec<String>>) -> Result<(), Box<dyn Error>> {
    let bytes = ByteStream::from(serde_json::to_vec(&tickers)?);
    put_object(BUCKET, TICKERS_FILE, bytes).await?;
    Ok(())
}

pub async fn get_company_names() -> ByteStream {
    get_object(BUCKET, COMPANIES_FILE).await
}

pub async fn put_company_names(names: HashMap<u64, Vec<String>>) -> Result<(), Box<dyn Error>> {
    let bytes = ByteStream::from(serde_json::to_vec(&names)?);
    put_object(BUCKET, COMPANIES_FILE, bytes).await?;
    Ok(())
}

async fn get_object(bucket: &str, key: &str) -> ByteStream {
    let client = client().await.unwrap();
    client.get_object()
        .bucket(bucket)
        .key(key)
        .send().await.unwrap().body
}

async fn put_object(bucket: &str, key: &str, bytes: ByteStream) -> Result<(), Box<dyn Error>> {
    let client = client().await?;
    client.put_object()
        .bucket(bucket)
        .key(key)
        .body(bytes)
        .send().await?;
    Ok(())
}