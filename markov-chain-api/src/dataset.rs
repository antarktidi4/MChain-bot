use std::collections::HashMap;

static FILENAME: &str = "dataset.mc";

pub fn load() -> HashMap<String, Vec<String>> {
  let content = std::fs::read_to_string(FILENAME).expect("");
  if content.len() == 0 { return HashMap::<String, Vec<String>>::new() }
  serde_json::from_str(&content).unwrap()
}

pub fn save(data: HashMap<String, Vec<String>>) {
  let serialized = serde_json::json!(data).to_string();
  std::fs::write(FILENAME, serialized);
}