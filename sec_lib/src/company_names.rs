use std::collections::HashMap;
use reqwest::{Response};

pub async fn request_lookup_data() -> Result<Response, Box<dyn std::error::Error>> {
    Ok(crate::http_client()?
        .get("https://www.sec.gov/Archives/edgar/cik-lookup-data.txt")
        .send()
        .await?)
}

pub async fn consume_response(response: Response) -> HashMap<u64, Vec<String>> {
    let text: String = response.text().await.unwrap();
    log::info!("Got text.");
    let mut names: HashMap<u64, Vec<String>> = HashMap::new();
    let mut lines = text.lines();
    while let Some(line) = lines.next() {
        let line_parts = parse_line(String::from(line));
        match line_parts {
            Some(parts) => update_map(&mut names, parts),
            None => ()
        }
    }
    log::info!("Built map.");
    log::info!("Keys: {:?}", names.keys().len());
    names
}

fn update_map(map: &mut HashMap<u64, Vec<String>>, parts: LineParts) {
    if map.contains_key(&parts.cik) {
        let v = map.get_mut(&parts.cik).unwrap();
        v.push(parts.name);
    } else {
        map.insert(parts.cik, vec!(parts.name));
    }
}

struct LineParts {
    name: String,
    cik: u64
}

impl LineParts {
    fn new(t: &str, c: u64) -> LineParts {
        LineParts { name: t.to_string(), cik: c}
    }
}

/*
 Parse a line in format: "example company name :cik:"
 */
fn parse_line(mut line: String) -> Option<LineParts> {
    // Trim the right colon.
    if !line.ends_with(':') {
        log::error!("Parse error. Skipping line which does not end with colon:. {:?}", line);
        return Option::None;
    }
    line.remove(line.len()-1);
    // Find second colon from right. This delimits name from CIK.
    let split_index = line.rfind(':');
    if split_index.is_none() {
        log::error!("Parse error. Cannot find split index: {:?}", line);
        return Option::None;
    }
    // Split line around index, first section is the company name, this is converted to a string and
    // trimmed. The second section is the 10 digit Central Index Key (CIK), this is converted to u64.
    let (name_str, cik_str) = line.split_at(split_index?);
    let name = String::from(name_str);
    let mut cs = String::from(cik_str);
    cs.remove(0);
    let cik = cs.parse::<u64>().unwrap();
    Option::Some(LineParts::new(name.trim(), cik))
}
