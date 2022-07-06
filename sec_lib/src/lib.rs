use std::error::Error;
use reqwest::{Client, ClientBuilder};
use reqwest::header::{USER_AGENT, ACCEPT_ENCODING, HOST, HeaderMap};

pub mod tickers;
pub mod company_names;
pub mod resources;
mod parse_facts;

fn http_client() -> Result<Client, Box<dyn Error>>{
    let client: Client = ClientBuilder::new()
        .gzip(true)
        .default_headers(sec_headers())
        .build()?;
    Ok(client)
}

pub fn sec_headers() -> HeaderMap {
    let mut map = HeaderMap::new();
    map.append(USER_AGENT, "Cash and Cash Equivalents admin@cashandcashequivalents.com".parse().unwrap());
    map.append(ACCEPT_ENCODING, "gzip, deflate".parse().unwrap());
    map.append(HOST, "www.sec.gov".parse().unwrap());
    map
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use std::future::Future;
    use log::LevelFilter;
    use crate::parse_facts::{AAPL_FACTS, AMZN_FACTS, FB_FACTS, GOOG_FACTS, MSFT_FACTS, NFLX_FACTS, open_file};
    use serde_json::json;

    /*
    TODO: load a few XBRL docs and see which GAAP labels overlap
     */
    // #[test]
    // fn it_works() {
    //     simple_logger::SimpleLogger::new().with_utc_timestamps()
    //         .with_level(LevelFilter::Info).init().unwrap();
    //     let amzn = open_file(AMZN_FACTS);
    //
    //     let mut common_labels: HashSet<String> = HashSet::new();
    //     log::info!("US-GAAP");
    //     // First populate set with any one of the companies.
    //     for (label, facts) in amzn.facts.gaap {
    //         common_labels.insert(label);
    //     }
    //     // Now union with the other companies.
    //     for data_file in vec!(FB_FACTS, AAPL_FACTS, NFLX_FACTS, GOOG_FACTS, MSFT_FACTS) {
    //         let filer_data = open_file(data_file);
    //         let mut new_labels = HashSet::new();
    //         for (label, _) in filer_data.facts.gaap {
    //             new_labels.insert(label);
    //         }
    //         log::info!("{} {}", data_file, new_labels.len());
    //         common_labels = common_labels.intersection(&new_labels).map(|s| String::from(s)).collect();
    //     }
    //
    //     let mut vec_labels: Vec<String> = common_labels.into_iter().collect();
    //     log::info!("num common facts: {}", vec_labels.len());
    //     vec_labels.sort();
    //     for label in &vec_labels {
    //         // log::info!("{:?}", label);
    //     }
    //     log::info!("{}", serde_json::to_string(&vec_labels).unwrap())
    // }

    #[test]
    fn it_works() {
        simple_logger::SimpleLogger::new().with_utc_timestamps()
            .with_level(LevelFilter::Info).init().unwrap();
        let amzn = open_file(AMZN_FACTS);
        // 36,393,000,000
        let mut common_labels: HashSet<String> = HashSet::new();
        log::info!("US-GAAP");
        let cash = amzn.facts.gaap.get("CashAndCashEquivalentsAtCarryingValue").unwrap();
        let units = cash.units.get("USD").unwrap();

        // for fact in units {
        //     log::info!("{} {} {} {} {} {}", fact.fy, fact.fp, fact.val, fact.filed, fact.end, fact.form);
        // }

        log::info!("{}", amzn.labels().len());

        log::info!("{}", json!(amzn.get_fact_history("EarningsPerShareDiluted".to_string())))
        // First populate set with any one of the companies.
        // for (label, facts) in amzn.facts.gaap {
        //     common_labels.insert(label);
        // }
        // // Now union with the other companies.
        // for data_file in &[FB_FACTS, AAPL_FACTS, NFLX_FACTS, GOOG_FACTS, MSFT_FACTS] {
        //     let filer_data = open_file(data_file);
        //     let mut new_labels = HashSet::new();
        //     for (label, _) in filer_data.facts.gaap {
        //         new_labels.insert(label);
        //     }
        //     log::info!("{} {}", data_file, new_labels.len());
        //     common_labels = common_labels.intersection(&new_labels).map(|s| String::from(s)).collect();
        // }
        //
        // let mut vec_labels: Vec<String> = common_labels.into_iter().collect();
        // log::info!("num common facts: {}", vec_labels.len());
        // vec_labels.sort();
        // for label in &vec_labels {
        //     // log::info!("{:?}", label);
        // }
        // log::info!("{}", serde_json::to_string(&vec_labels).unwrap())
    }

    // #[tokio::test]
    // async fn company_names() {
    //     simple_logger::SimpleLogger::new().with_utc_timestamps()
    //         .with_level(LevelFilter::Info).init().unwrap();
    //     log::info!("Start.");
    //     let response = match crate::company_names::request_lookup_data().await {
    //         Ok(str) => str,
    //         Err(e) => panic!("Failed to retrieve document: {:?}", e)
    //     };
    //     log::info!("Got response.");
    //     let map = crate::company_names::consume_response(response).await;
    //     crate::resources::put_company_names(map).await;
    //     log::info!("Finish.");
    // }
}
