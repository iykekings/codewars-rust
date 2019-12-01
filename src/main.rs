mod meeting;
use meeting::meeting;

fn main() {
  let ex = "Fred:Corwill;Wilfred:Corwill;Barney:Tornbull;Betty:Tornbull;Bjon:Tornbull;Raphael:Corwill;Alfred:Corwill";

  let result = meeting(ex);
  println!("{}", result);
}
