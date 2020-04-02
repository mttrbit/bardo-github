use reqwest::header::{HeaderMap, HeaderValue};
use std::str::FromStr;
use std::collections::HashMap;

use regex::Regex;

/// Extract however many requests the authenticated user can
/// make from the Headers
pub fn link(head: &HeaderMap) -> Option<HashMap<String, HashMap<String, String>>> {
    head.get("link").map(|l| parse(l.to_str().unwrap_or("")))
}

fn parse(link_header: &str) -> HashMap<String, HashMap<String, String>> {
    let mut result: HashMap<String, HashMap<String, String>> = HashMap::new();
    let re = Regex::new(r#"[<>"\s]"#).unwrap();
    let preprocessed = re.replace_all(link_header, "");
    let splited = preprocessed.split(',');

    for s in splited {
        let mut link_vec: Vec<&str> = s.split(";").collect();
        link_vec.reverse();

        let link_val = link_vec.pop().unwrap();
        let url_parsed = reqwest::Url::parse(link_val).unwrap();
        let query_pairs = url_parsed.query_pairs();

        let mut rel_val = String::from("");
        let mut map: HashMap<String, String> = HashMap::new();

        for param in link_vec {
            let param_kv: Vec<&str> = param.split("=").collect();
            let key = param_kv[0];
            let val = param_kv[1];

            if key == "rel" {
                rel_val = val.to_string();
            }

            map.insert(key.to_string(), val.to_string());
        }

        for pair in query_pairs {
            map.insert(pair.0.to_string(), pair.1.to_string());
        }

        map.insert("link".to_string(), link_val.to_string());

        result.insert(rel_val, map);
    }

    result
}
