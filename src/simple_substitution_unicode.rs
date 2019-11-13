extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct Cipher<'a> {
  map: Vec<(&'a str, &'a str)>
}

impl<'a> Cipher<'a> {
  pub fn new(map1: &'a str, map2: &'a str) -> Cipher<'a> {
    Cipher { map: UnicodeSegmentation::graphemes(map1, true).zip(UnicodeSegmentation::graphemes(map2, true)).collect() }
  }
  pub fn encode(&self, string: &str) -> String {
    UnicodeSegmentation::graphemes(string, true)
      .map(|c| self.map.iter().find(|x| x.0 == c)
      .map_or(c, |y| y.1)).collect()
  }
  pub fn decode(&self, string: &str) -> String {
    UnicodeSegmentation::graphemes(string, true)
      .map(|c| self.map.iter().find(|x| x.1 == c)
      .map_or(c, |y| y.0)).collect()
  }
}
#[test]
fn test() {
  let map1 = "abcdefghijklmnopqrstuvwxyz🔥";
  let map2 = "etaoinshrdlucmfwypvbgkjqxz💥";

  let cipher = Cipher::new(map1, map2);
  assert_eq!(cipher.encode("abc"), "eta");
  assert_eq!(cipher.encode("xyz🔥"), "qxz💥");
  assert_eq!(cipher.decode("eirfg"), "aeiou");
  assert_eq!(cipher.decode("erlang"), "aikcfu");
}
