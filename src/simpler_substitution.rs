#[derive(Debug)]
pub struct Cipher {
  map: Vec<(char, char)>,
}

impl Cipher {
  #[allow(dead_code)]
  pub fn new(map1: &str, map2: &str) -> Cipher {
    Cipher {
      map: map1.chars().zip(map2.chars()).collect(),
    }
  }
  #[allow(dead_code)]
  pub fn encode(&self, string: &str) -> String {
    string
      .chars()
      .map(|c| self.map.iter().find(|x| x.0 == c).map_or(c, |y| y.1))
      .collect()
  }
  #[allow(dead_code)]
  pub fn decode(&self, string: &str) -> String {
    string
      .chars()
      .map(|c| self.map.iter().find(|x| x.1 == c).map_or(c, |y| y.0))
      .collect()
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
