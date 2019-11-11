extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashMap;

pub struct Cipher {
  dict1: HashMap<String, String>,
  dict2: HashMap<String, String>
}

impl Cipher {
  pub fn new(map1: &str, map2: &str) -> Cipher {
    let mut dictx: HashMap<String, String> = HashMap::new();
    let mut dicty: HashMap<String, String> = HashMap::new();
    let target = UnicodeSegmentation::graphemes(map2, true).collect::<Vec<&str>>();
    let value = UnicodeSegmentation::graphemes(map1, true).collect::<Vec<&str>>();
    for i in 0..value.len() {
      dictx.insert(value[i].to_string(), target[i].to_string());
    }
    for i in 0..target.len() {
      dicty.insert(target[i].to_string(), value[i].to_string());
    }
    Cipher { dict1: dictx, dict2: dicty }
  }
  pub fn encode(&self, string: &str) -> String {
    UnicodeSegmentation::graphemes(string, true)
      .map(|x| self.dict1.get(x).unwrap().clone())
      .collect::<Vec<String>>().join("")
  }
  pub fn decode(&self, string: &str) -> String {
    UnicodeSegmentation::graphemes(string, true)
      .map(|x| self.dict2.get(x).unwrap().clone())
      .collect::<Vec<String>>().join("")
  }
}
#[test]
fn test() {
  let map1 = "abcdefghijklmnopqrstuvwxyz";
  let map2 = "etaoinshrdlucmfwypvbgkjqxz";

  let cipher = Cipher::new(map1, map2);
  assert_eq!(cipher.encode("abc"), "eta");
  assert_eq!(cipher.encode("xyz"), "qxz");
  assert_eq!(cipher.decode("eirfg"), "aeiou");
  assert_eq!(cipher.decode("erlang"), "aikcfu");
}
