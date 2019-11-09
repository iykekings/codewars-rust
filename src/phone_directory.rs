extern crate regex;
use regex::Regex;

pub fn phone(dir: &str, num: &str) -> String {
let nnum = format!("+{}", num);
let contact: Vec<&str> = dir.split("\n").filter(|s| s.contains(nnum.as_str())).collect();
if contact.len() < 1 {
  return format!("Error => Not found: {}", num);
}
if contact.len() > 1 {
  return format!("Error => Too many people: {}", num);
}
let constr = contact[0];
let phone_re = Regex::new(r"[^\w<>]?\s*?\+(\d{1,2}-\d{3}-\d{3}-\d{4})[^\w<>]?").unwrap();
let constr = &phone_re.replace(constr, "").to_string();
let name_re = Regex::new(r"<(.*)>").unwrap();
let name = name_re.captures(constr).unwrap();
let addr = &name_re.replace(constr, "").to_string().replace("_", " ").replace("  ", " ");
format!("Phone => {}, Name => {}, Address => {}", num, &name[1], addr.trim())

}