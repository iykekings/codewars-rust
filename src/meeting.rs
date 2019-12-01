pub fn meeting(s: &str) -> String {
  let mut ans: Vec<String> = s
    .to_uppercase()
    .split(";")
    .map(|x| {
      let m: Vec<&str> = x.split(":").collect();
      format!("({}, {})", m[1], m[0])
    })
    .collect();
  ans.sort();
  ans.join("")
}
