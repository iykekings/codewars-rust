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

// assert_eq!(hist("tpwaemuqxdmwqbqrjbeosjnejqorxdozsxnrgpgqkeihqwybzyymqeazfkyiucesxwutgszbenzvgxibxrlvmzihcb"), "u  3     ***\rw  4     ****\rx  6     ******\rz  6     ******");
