#[allow(dead_code)]
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

#[test]
fn meeting_works() {
  let ex_str = "Fred:Corwill;Wilfred:Corwill;Barney:Tornbull;Betty:Tornbull;Bjon:Tornbull;Raphael:Corwill;Alfred:Corwill";
  let expected = String::from("(CORWILL, ALFRED)(CORWILL, FRED)(CORWILL, RAPHAEL)(CORWILL, WILFRED)(TORNBULL, BARNEY)(TORNBULL, BETTY)(TORNBULL, BJON)");
  assert_eq!(meeting(ex_str), expected);
}
