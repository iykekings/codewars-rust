pub struct Cipher {
  map1: String,
  map2: String
}

impl Cipher {
  pub fn new(map1: &str, map2: &str) -> Cipher {
    Cipher { map1: map1.to_string(), map2: map2.to_string()}
  }
  #[allow(dead_code)]
  pub fn encode(&self, string: &str) -> String {
    let vec: Vec<usize> = string.chars().map(|x| self.map1.find(x).unwrap()).collect();
    let res: Vec<String> = vec.into_iter()
      .map(|x| self.map2.chars().nth(x).unwrap()
      .to_string()).collect();
    res.join("")
  }
  #[allow(dead_code)]
  pub fn decode(&self, string: &str) -> String {
    let vec: Vec<usize> = string.chars().map(|x| self.map2.find(x).unwrap()).collect();
    let res: Vec<String> = vec.into_iter()
      .map(|x| self.map1.chars().nth(x).unwrap()
      .to_string()).collect();
    res.join("")
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
