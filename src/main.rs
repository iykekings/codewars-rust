mod simple_substitution;
use simple_substitution::Cipher;

fn main() {
  let map1 = "abcdefghijklmnopqrstuvwxyz";
  let map2 = "etaoinshrdlucmfwypvbgkjqxz";

  let cipher = Cipher::new(map1, map2);
  
  println!("{}", cipher.encode("abc"));
  println!("{}", cipher.decode("erlang"));
}
