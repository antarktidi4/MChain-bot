use std::{collections::HashMap, vec};

use crate::dataset;

pub struct MarkovChain {
  word_association_map: HashMap<String, Vec<String>>
}

impl MarkovChain {
  pub fn new() -> Self {
    Self {
      word_association_map: dataset::load()
    }
  }

  pub fn append(&mut self, sentence: &String) {
    let words = sentence.to_lowercase().split(" ").map(String::from).collect::<Vec<String>>();
    self.append_words(words);
  }

  pub fn generate(&mut self, word_count: u16) -> String {
    // I don't like this...
    let map_keys: Vec<&String> = self.word_association_map.keys().collect();
    if map_keys.len() == 0 { return String::from("no data"); }
    
    let index = (rand::random::<f32>() * map_keys.len() as f32).floor() as usize;
    let mut word = map_keys.get(index).unwrap().as_str();
    
    let mut chain: Vec<&str> = vec![word];
    
    for _ in 0..word_count {
      let next_words_list = self.word_association_map.get(word);
      if next_words_list.is_none() { break; }

      let index = (rand::random::<f32>() * next_words_list.unwrap().len() as f32).floor() as usize;
      word = next_words_list.unwrap().get(index).unwrap().as_str();
      
      chain.push(word);
    }

    chain.join(" ")
  }

  fn append_words(&mut self, words: Vec<String>) {
    if words.len() == 0 { return; }
    let mut words_iter = words.iter().peekable();

    for _ in 0..words.len() {
      let first_word = words_iter.next().unwrap();
      let second_word = words_iter.peek();

      if second_word.is_none() { continue; }

      self.word_association_map
          .entry(first_word.clone())
          .or_insert(Vec::<String>::new())
          .push(second_word.unwrap().clone().to_string());
    }
  }
}

impl Drop for MarkovChain {
  fn drop(&mut self) {
    dataset::save(self.word_association_map.clone());
  }
}