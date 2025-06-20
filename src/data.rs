use std::collections::HashMap;
use std::fs;

pub struct Data {
    pub levels: HashMap<u8, Vec<char>>,
    pub stroke_counts: HashMap<u8, Vec<char>>,
}

impl Data {
    pub fn new() -> Self {
        let levels: HashMap<u8, Vec<char>> = load_json::<HashMap<String, String>>("levels.json")
            .into_iter()
            .map(|(k, v)| (k.parse::<u8>().unwrap(), v.chars().collect()))
            .collect();

        let stroke_counts: HashMap<u8, Vec<char>> =
            load_json::<HashMap<String, String>>("stroke-count.json")
                .into_iter()
                .map(|(k, v)| (k.parse::<u8>().unwrap(), v.chars().collect()))
                .collect();

        Self {
            levels,
            stroke_counts,
        }
    }
}

fn load_json<T: for<'de> serde::Deserialize<'de>>(file: &str) -> HashMap<String, String> {
    let content = fs::read_to_string(file).expect("Failed to read file");
    serde_json::from_str(&content).expect("Failed to parse JSON")
}
