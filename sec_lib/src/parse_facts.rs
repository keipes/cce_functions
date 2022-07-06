use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde::{Serialize, Deserialize};

pub const MSFT_FACTS: &str = "resources/test-data/CIK0000789019.json";
pub const FB_FACTS: &str = "resources/test-data/CIK0001326801.json";
pub const AMZN_FACTS: &str = "resources/test-data/CIK0001018724.json";
pub const AAPL_FACTS: &str = "resources/test-data/CIK0000320193.json";
pub const NFLX_FACTS: &str = "resources/test-data/CIK0001065280.json";
pub const GOOG_FACTS: &str = "resources/test-data/CIK0001652044.json";

/*
TODO: Can we enable strict parsing, so we know when an element is not successfully mapped to a data
 structure?
 */
pub fn open_file(file: &str) -> Filer {
    let file = File::open(Path::new(file)).unwrap();
    let mut buf_reader = BufReader::new(file);
    serde_json::from_reader(buf_reader).unwrap()
}

// in
#[derive(Serialize, Deserialize)]
pub struct Filer {
    pub cik: u64,
    pub entityName: String,
    pub facts: DocumentEntityInformation
}

impl Filer {
    pub fn labels(&self) -> Vec<String> {
        let mut labels: Vec<String> = Vec::new();
        for (label, facts) in &self.facts.gaap {
            // std::println!("{}", label);
            labels.push(label.to_string());
        }
        labels
    }

    pub fn get_fact_history(&self, label: String) -> Option<FactHistory> {
        let element_maybe = self.facts.gaap.get(label.as_str());
        if element_maybe.is_some() {
            let element = element_maybe?;
            if element.units.len() > 1 {
                panic!("multiple units for one fact") // TODO: removeme
            }
            let mut fact_data: Vec<FactPoint> = Vec::new();
            let mut unit_label= "";
            let mut map: HashMap<String, f64> = HashMap::new();
            for (unit, facts) in &element.units {
                unit_label = unit;
                for fact in facts {
                    let mut p: &str = fact.fp.as_str();
                    if (p == "FY") {
                        p = "Q4";
                    }
                    map.insert(format!("{} {}", fact.fy, p), fact.val);
                    // fact_data.push(FactPoint {
                    //     time: format!("{} {}", fact.fy, p),
                    //     value: fact.val
                    // })
                }
            }
            for (label, val) in map {
                fact_data.push(FactPoint { time: label, value: val});
            }
            fact_data.sort_by_key(|fp| fp.time.to_string());

            Some(FactHistory {
                label: label,
                unit: unit_label.to_string(),
                data: fact_data
            })
        } else {
            None
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DocumentEntityInformation {
    pub dei: HashMap<String, FinancialElement>, // do you like this map structure? is it the only way to map fields with variable keys to a struct? what about the "other" annotation in serde
    #[serde(alias = "us-gaap")]
    pub gaap: HashMap<String, FinancialElement>
}

#[derive(Serialize, Deserialize)]
pub struct FinancialElement {
    pub label: String,
    pub description: String,
    pub units: HashMap<String, Vec<Fact>>
}

#[derive(Serialize, Deserialize)]
pub struct Fact {
    #[serde(default)]
    pub start: String, // TODO: make this a date?
    pub end: String, // TODO: make this a date?
    pub val: f64, // floating point? I don't think so
    pub accn: String, // accession number? (doc which fact is from)
    pub fy: u32, // fiscal year
    pub fp: String, // fiscal period (Q1/Q2/Q3/FY)
    pub form: String,
    pub filed: String // TODO: make this a date!
}

// out
#[derive(Serialize)]
pub struct FactHistory {
    pub label: String,
    pub unit: String,
    // pub x_label: String,
    // pub y_label: String,
    pub data: Vec<FactPoint>
}

#[derive(Serialize)]
pub struct FactPoint {
    pub time: String,
    pub value: f64
}

// impl FactHistory {
//     pub fn
// }