use std::collections::HashMap;

#[allow(dead_code)]
pub struct Cipher {
  dict1: HashMap<String, String>,
  dict2: HashMap<String, String>,
}

impl Cipher {
  #[allow(dead_code)]
  pub fn new(map1: &str, map2: &str) -> Cipher {
    let mut dictx: HashMap<String, String> = HashMap::new();
    let mut dicty: HashMap<String, String> = HashMap::new();
    let target: Vec<char> = map2.chars().collect();
    let value: Vec<char> = map1.chars().collect();
    for i in 0..value.len() {
      dictx.insert(value[i].to_string(), target[i].to_string());
    }
    for i in 0..target.len() {
      dicty.insert(target[i].to_string(), value[i].to_string());
    }
    Cipher {
      dict1: dictx,
      dict2: dicty,
    }
  }
  #[allow(dead_code)]
  pub fn encode(&self, string: &str) -> String {
    string
      .chars()
      .map(|x| {
        self
          .dict1
          .get(&x.to_string())
          .unwrap_or(&x.to_string())
          .clone()
      })
      .collect::<Vec<String>>()
      .join("")
  }
  #[allow(dead_code)]
  pub fn decode(&self, string: &str) -> String {
    string
      .chars()
      .map(|x| {
        self
          .dict2
          .get(&x.to_string())
          .unwrap_or(&x.to_string())
          .clone()
      })
      .collect::<Vec<String>>()
      .join("")
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn encoding() {
    let map1 = "abcdefghijklmnopqrstuvwxyz🔥";
    let map2 = "etaoinshrdlucmfwypvbgkjqxz💥";
    let cipher = Cipher::new(map1, map2);
    assert_eq!(cipher.encode("abc"), "eta");
    assert_eq!(cipher.encode("xyz"), "qxz");
    assert_eq!(cipher.encode("xyz🔥"), "qxz💥");
  }
  #[test]
  fn decoding() {
    let map1 = "abcdefghijklmnopqrstuvwxyz🔥";
    let map2 = "etaoinshrdlucmfwypvbgkjqxz💥";
    let cipher = Cipher::new(map1, map2);
    assert_eq!(cipher.decode("eirfg"), "aeiou");
    assert_eq!(cipher.decode("erlang"), "aikcfu");
  }
}
