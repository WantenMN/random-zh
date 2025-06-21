use std::collections::HashMap;

pub struct Data {
    pub levels: HashMap<u8, Vec<char>>,
    pub stroke_counts: HashMap<u8, Vec<char>>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            levels: Self::load_and_parse_json(include_str!("../levels.json")),
            stroke_counts: Self::load_and_parse_json(include_str!("../stroke-count.json")),
        }
    }

    fn load_and_parse_json(content: &str) -> HashMap<u8, Vec<char>> {
        load_json::<HashMap<String, String>>(content)
            .into_iter()
            .map(|(k, v)| {
                (
                    k.parse::<u8>().expect("Invalid key format"),
                    v.chars().collect(),
                )
            })
            .collect()
    }
}

fn load_json<T: for<'de> serde::Deserialize<'de>>(content: &str) -> T {
    serde_json::from_str(content).expect("Failed to parse JSON")
}
