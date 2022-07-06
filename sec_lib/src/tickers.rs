use std::collections::HashMap;
use reqwest::{Response};
use log;

const _SEC_TICKER_URL: &str = "https://www.sec.gov/include/ticker.txt";

pub async fn get_tickers() -> Result<Response, Box<dyn std::error::Error>> {
    Ok(crate::http_client()?
        .get("https://www.sec.gov/include/ticker.txt")
        .send()
        .await?)
}

pub async fn parse_tickers(response: Response) -> HashMap<u64, Vec<String>> {
    let mut ciks: HashMap<u64, Vec<String>> = HashMap::new();
    let text: String = response.text().await.unwrap();
    log::info!("Got text.");
    let mut lines = text.lines();
    while let Some(line) = lines.next() {
        let line_parts = parse_line(&line);
        match line_parts {
            Some(parts) => update_map(&mut ciks, parts),
            None => ()
        }
    }
    log::info!("Built map.");
    log::info!("CIKs: {:?}", ciks.keys().len());
    ciks
}

fn update_map(map: &mut HashMap<u64, Vec<String>>, parts: CikTicker) {
    if map.contains_key(&parts.cik) {
        let v = map.get_mut(&parts.cik).unwrap();
        v.push(parts.ticker);
    } else {
        map.insert(parts.cik, vec!(parts.ticker));
    }
}

struct CikTicker {
    ticker: String,
    cik: u64
}

impl CikTicker {
    fn new(t: &str, c: u64) -> CikTicker {
        CikTicker { ticker: t.to_string(), cik: c}
    }
}

fn parse_line(line: &str) -> Option<CikTicker> {
    let mut split = line.split_whitespace();
    let ticker_maybe = split.next();
    let cik_maybe = split.next();
    if ticker_maybe.is_none() || cik_maybe.is_none() {
        log::error!("parse error: {:?}", line);
        return Option::None;
    } else {
        let ticker = ticker_maybe.unwrap();
        let cik = cik_maybe.unwrap().parse::<u64>().unwrap();
        Option::Some(CikTicker::new(ticker, cik))
    }
}
