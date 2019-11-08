extern crate regex;
use regex::Regex;

pub fn phone(dir: &str, num: &str) -> String {

let contact: Vec<&str> = dir.split("\n").filter(|s| s.contains(num)).collect();
let constr = contact[0];
let phone_re = Regex::new(r"\W?\s*?\+(\d-\d{3}-\d{3}-\d{4})\W?").unwrap();
let phone = phone_re.captures(constr).unwrap();
let constr = &phone_re.replace(constr, "").to_string();
let name_re = Regex::new(r"<(.*)>").unwrap();
let name = name_re.captures(constr).unwrap();
let addr = &name_re.replace(constr, "").to_string();
format!("Phone => {}, Name => {}, Address => {}", &phone[1], &name[1], addr.trim())

}