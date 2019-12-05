#[allow(dead_code)]
pub fn hist(s: &str) -> String {
  let error = vec!["u", "w", "x", "z"];
  let mut collection: Vec<String> = vec![];

  for err in error.iter() {
    let count = s.matches(err).count();
    let result: String;
    if count > 0 {
      result = format!("{}  {}     {}", err, count, "*".repeat(count));
      collection.push(result);
    }
  }
  println!("{:?}", collection);
  format!("{}", collection.join("\r"))
}

#[allow(dead_code)]
pub fn hist2(s: &str) -> String {
  ["u", "w", "x", "z"]
    .iter()
    .fold(vec![], |mut acc, err| {
      let count = s.matches(err).count();
      if count > 0 {
        acc.push(format!("{}  {}     {}", err, count, "*".repeat(count)));
      }
      acc
    })
    .join("\r")
}

#[cfg(test)]
mod tests {
  use super::{hist, hist2};
  #[test]
  fn histogram_works() {
    assert_eq!(hist("tpwaemuqxdmwqbqrjbeosjnejqorxdozsxnrgpgqkeihqwybzyymqeazfkyiucesxwutgszbenzvgxibxrlvmzihcb"), "u  3     ***\rw  4     ****\rx  6     ******\rz  6     ******");
    assert_eq!(hist2("tpwaemuqxdmwqbqrjbeosjnejqorxdozsxnrgpgqkeihqwybzyymqeazfkyiucesxwutgszbenzvgxibxrlvmzihcb"), "u  3     ***\rw  4     ****\rx  6     ******\rz  6     ******");
  }
}
